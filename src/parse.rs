use std::str::CharIndices;
use std::char;

use lex::Lex;
use json::Json;
use kv::{self, KeyValue};

pub fn parse_value(text: &str, lex: &mut Lex) -> Result<Json,String> {
    parse_whitespace(text, lex);

    check_eof(text, lex)?;

    //println!("text -- {:?}", valtext);
    match (&text[lex.off..]).as_bytes()[0] {
        b'n' => parse_null(text, lex),
        b't' => parse_true(text, lex),
        b'f' => parse_false(text, lex),
        b'0'..=b'9'|b'+'|b'-'|b'.'|b'e'|b'E' => parse_num(text, lex),
        b'"' => parse_string(text, lex),
        b'[' => parse_array(text, lex),
        b'{' => parse_object(text, lex),
        ch => Err(lex.format(&format!("parse: invalid token {}", ch))),
    }
    //println!("valu -- {:?}", v);
}

fn parse_null(text: &str, lex: &mut Lex) -> Result<Json,String> {
    let text = &text[lex.off..];
    if text.len() >= 4 && &text[..4] == "null" {
        lex.incr_col(4);
        Ok(Json::Null)
    } else {
        Err(lex.format("parse: expected null"))
    }
}

fn parse_true(text: &str, lex: &mut Lex) -> Result<Json,String> {
    let text = &text[lex.off..];
    if text.len() >= 4 && &text[..4] == "true" {
        lex.incr_col(4);
        Ok(Json::Bool(true))
    } else {
        Err(lex.format("parse: expected true"))
    }
}

fn parse_false(text: &str, lex: &mut Lex) -> Result<Json,String> {
    let text = &text[lex.off..];
    if text.len() >= 5 && &text[..5] == "false" {
        lex.incr_col(5);
        Ok(Json::Bool(false))
    } else {
        Err(lex.format("parse: expected false"))
    }
}

fn parse_num(text: &str, lex: &mut Lex) -> Result<Json,String> {
    let text = &text[lex.off..];

    let mut doparse = |text: &str, i: usize, f: bool| -> Result<Json,String> {
        lex.incr_col(i);
        if f {
            text.parse::<f64>()
                .map(|v| Json::Float(v))
                .map_err(|_| format!("parse: invalid {}", text))
        } else {
            text.parse::<i128>()
                .map(|v| Json::Integer(v))
                .map_err(|_| format!("parse: invalid {}", text))
        }
    };

    let mut is_float = false;
    for (i, ch) in text.char_indices() {
        match ch {
            '0'..='9'|'+'|'-' => continue, // valid number
            '.'|'e'|'E' => { is_float = true; continue}, // float number
            _ => (),
        }
        return doparse(&text[..i], i, is_float)
    }
    doparse(text, text.len(), is_float)
}

fn parse_string(text: &str, lex: &mut Lex) -> Result<Json,String> {
    use self::Json::{String as S};

    let mut escape = false;
    let mut res = String::new();
    let mut chars = (&text[lex.off..]).char_indices();

    let (i, ch) = chars.next().unwrap(); // skip the opening quote
    if ch != '"' {
        return Err(lex.format("parse: not a string"))
    }

    while let Some((i, ch)) = chars.next() {
        if escape == false {
            if ch == '\\' {
                escape = true;
                continue
            }
            match ch {
                '"' => {
                    lex.incr_col(i+1);
                    return Ok(S(res));
                },
                _ => res.push(ch),
            }
            continue
        }

        // previous char was escape
        match ch {
            '"' => res.push('"'),
            '\\' => res.push('\\'),
            '/' => res.push('/'),
            'b' => res.push('\x08'),
            'f' => res.push('\x0c'),
            'n' => res.push('\n'),
            'r' => res.push('\r'),
            't' => res.push('\t'),
            'u' => match decode_json_hex_code(&mut chars, lex)? {
                code1 @ 0xDC00 ... 0xDFFF => {
                    lex.incr_col(i);
                    let err = format!("parse: invalid string codepoint {}", code1);
                    return Err(lex.format(&err))
                },
                // Non-BMP characters are encoded as a sequence of
                // two hex escapes, representing UTF-16 surrogates.
                code1 @ 0xD800 ... 0xDBFF => {
                    let code2 = decode_json_hex_code2(&mut chars, lex)?;
                    if code2 < 0xDC00 || code2 > 0xDFFF {
                        lex.incr_col(i);
                        let err = format!("parse: invalid string codepoint {}", code2);
                        return Err(lex.format(&err))
                    }
                    let code = (((code1 - 0xD800) as u32) << 10 |
                                 (code2 - 0xDC00) as u32) + 0x1_0000;
                    res.push(char::from_u32(code).unwrap());
                },

                n => match char::from_u32(n as u32) {
                    Some(ch) => res.push(ch),
                    None => {
                        lex.incr_col(i);
                        let err = format!("parse: invalid string escape code {:?}", n);
                        return Err(lex.format(&err))
                    },
                },
            },
            _ => {
                lex.incr_col(i);
                let err = "parse: invalid string string escape type";
                return Err(lex.format(&err))
            },
        }
        escape = false;
    }
    lex.incr_col(i);
    return Err(lex.format("parse: incomplete string"))
}

fn decode_json_hex_code2(chars: &mut CharIndices, lex: &mut Lex)
    -> Result<u32,String>
{
    if let Some((_, ch1)) = chars.next() {
        if let Some((_, ch2)) = chars.next() {
            if ch1 == '\\' && ch2 == 'u' {
                return decode_json_hex_code(chars, lex)
            }
        }
    }
    let err = "parse: invalid string string escape type";
    return Err(lex.format(err))
}

fn decode_json_hex_code(chars: &mut CharIndices, lex: &mut Lex)
    -> Result<u32,String>
{
    let mut n = 0;
    let mut code = 0_u32;
    while let Some((_, ch)) = chars.next() {
        if (ch as u8) > 128 || HEXNUM[ch as usize] == 20 {
            let err = format!("parse: invalid string escape code {:?}", ch);
            return Err(lex.format(&err))
        }
        code = code * 16 + (HEXNUM[ch as usize] as u32);
        n += 1;
        if n == 4 {
            break
        }
    }
    if n != 4 {
        let err = format!("parse: incomplete string escape code {:x}", code);
        return Err(lex.format(&err))
    }
    Ok(code)
}

fn parse_array(text: &str, lex: &mut Lex) -> Result<Json,String> {
    lex.incr_col(1); // skip '['

    let mut array: Vec<Json> = Vec::new();
    parse_whitespace(text, lex);
    if (&text[lex.off..]).as_bytes()[0] == b',' {
        return Err(lex.format("parse: expected ','"))
    }
    loop {
        if (&text[lex.off..]).as_bytes()[0] == b']' { // end of array.
            lex.incr_col(1);
            break Ok(Json::Array(array))
        }

        array.push(parse_value(text, lex)?);

        parse_whitespace(text, lex);
        if (&text[lex.off..]).as_bytes()[0] == b',' { // skip comma
            lex.incr_col(1);
            parse_whitespace(text, lex);
        }
    }
}

fn parse_object(text: &str, lex: &mut Lex) -> Result<Json,String> {
    lex.incr_col(1); // skip '{'

    parse_whitespace(text, lex);

    let mut m: Vec<KeyValue> = Vec::new();

    if (&text[lex.off..]).as_bytes()[0] == b'}' {
        lex.incr_col(1);
        return Ok(Json::Object(m))
    }

    loop {
        // key
        parse_whitespace(text, lex);
        let key: String = parse_string(text, lex)?.string().unwrap();
        // colon
        parse_whitespace(text, lex);
        check_next_byte(text, lex, b':')?;

        // value
        parse_whitespace(text, lex);
        let value = parse_value(text, lex)?;

        kv::upsert_object_key(&mut m, KeyValue::new(key, value));
        //println!("parse {} {} {:?}", key, i, m);

        // is exit
        parse_whitespace(text, lex);
        if (&text[lex.off..]).len() == 0 {
            break Err(lex.format("parse: unexpected eof"))
        } else if (&text[lex.off..]).as_bytes()[0] == b'}' { // exit
            lex.incr_col(1);
            break Ok(Json::Object(m))
        } else if (&text[lex.off..]).as_bytes()[0] == b',' { // skip comma
            lex.incr_col(1);
        }
    }
}

fn parse_whitespace(text: &str, lex: &mut Lex) {
    for &ch in (&text[lex.off..]).as_bytes() {
        match WS_LOOKUP[ch as usize] {
            0 => break,
            1 => { lex.col += 1 },              // ' ' | '\t' | '\r'
            2 => { lex.row += 1; lex.col = 0 }, // '\n'
            _ => panic!("unreachable code"),
        };
        lex.off += 1;
    }
}

fn check_next_byte(text: &str, lex: &mut Lex, b: u8) -> Result<(),String> {
    let progbytes = (&text[lex.off..]).as_bytes();

    if progbytes.len() == 0 {
        return Err(lex.format(&format!("parse: missing token {}", b)));
    }
    if progbytes[0] != b {
        return Err(lex.format(&format!("parse: invalid token {}", b)));
    }
    lex.incr_col(1);

    Ok(())
}

fn check_eof(text: &str, lex: &mut Lex) -> Result<(),String> {
    if (&text[lex.off..]).len() == 0 {
        Err(lex.format("parse: unexpected eof"))
    } else {
        Ok(())
    }
}

static HEXNUM: [u8; 256] = [
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  0,  0,  0,  0,  0,  0,
    20, 10, 11, 12, 13, 14, 15,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    20, 20,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    20, 10, 11, 12, 13, 14, 15,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20
];

static WS_LOOKUP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];
