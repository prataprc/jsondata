use std::str::CharIndices;
use std::{char, f64};

use lazy_static::lazy_static;

use crate::json::Json;
use crate::lex::Lex;
use crate::num::{Floating, Integral};
use crate::property::{self, Property};

pub fn parse_value(text: &str, lex: &mut Lex) -> Result<Json, String> {
    parse_whitespace(text, lex);

    check_eof(text, lex)?;

    //println!("text -- {:?}", &text[lex.off..].as_bytes());
    let bs = (&text[lex.off..]).as_bytes();
    match bs[0] {
        b'n' => parse_null(text, lex),
        b't' => parse_true(text, lex),
        b'f' => parse_false(text, lex),
        b'-' if bs.len() > 1 && bs[1] == b'I' => parse_json5_float(text, lex, 1),
        b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E' => parse_num(text, lex),
        b'"' => parse_string(text, lex),
        b'[' => parse_array(text, lex),
        b'{' => parse_object(text, lex),
        b'I' => parse_json5_float(text, lex, 2),
        b'N' => parse_json5_float(text, lex, 3),
        ch => Err(lex.format(&format!("parse: invalid token {}", ch))),
    }
    //println!("valu -- {:?}", v);
}

fn parse_null(text: &str, lex: &mut Lex) -> Result<Json, String> {
    let text = &text[lex.off..];
    if text.len() >= 4 && &text[..4] == "null" {
        lex.incr_col(4);
        Ok(Json::Null)
    } else {
        Err(lex.format("parse: expected null"))
    }
}

fn parse_true(text: &str, lex: &mut Lex) -> Result<Json, String> {
    let text = &text[lex.off..];
    if text.len() >= 4 && &text[..4] == "true" {
        lex.incr_col(4);
        Ok(Json::Bool(true))
    } else {
        Err(lex.format("parse: expected true"))
    }
}

fn parse_false(text: &str, lex: &mut Lex) -> Result<Json, String> {
    let text = &text[lex.off..];
    if text.len() >= 5 && &text[..5] == "false" {
        lex.incr_col(5);
        Ok(Json::Bool(false))
    } else {
        Err(lex.format("parse: expected false"))
    }
}

fn parse_num(text: &str, lex: &mut Lex) -> Result<Json, String> {
    let text = &text[lex.off..];

    let mut doparse = |text: &str, i: usize, f: bool, h: bool| -> Result<Json, String> {
        lex.incr_col(i);
        //println!("parse_num -- {}", text);
        if f && !h {
            Ok(Json::Float(Floating::new(text)))
        } else {
            Ok(Json::Integer(Integral::new(text)))
        }
    };

    let mut is_float = false;
    let mut is_hex = false;
    for (i, ch) in text.char_indices() {
        if ISNUMBER[ch as usize] == 0 {
            return doparse(&text[..i], i, is_float, is_hex);
        } else if !is_float && ISNUMBER[ch as usize] == 2 {
            is_float = true
        } else if ISNUMBER[ch as usize] == 3 {
            is_hex = true
        }
    }
    doparse(text, text.len(), is_float, is_hex)
}

fn parse_json5_float(text: &str, lex: &mut Lex, w: usize) -> Result<Json, String> {
    lazy_static! {
        static ref JSON5_FLOAT_LOOKUP: Vec<(String, usize, Json)> = vec![
            ("".to_string(), 0_usize, Json::Null),
            ("-Infinity".to_string(), 9, Json::new(f64::NEG_INFINITY)),
            ("Infinity".to_string(), 8, Json::new(f64::INFINITY)),
            ("NaN".to_string(), 3, Json::new(f64::NAN)),
        ];
    }
    let (token, l, res) = &JSON5_FLOAT_LOOKUP[w];
    let text = &text[lex.off..];
    if text.len() >= *l && token == &text[..*l] {
        lex.col += l;
        lex.off += l;
        Ok(res.clone())
    } else {
        Err(lex.format(&format!("parse: expected {}", token)))
    }
}

fn parse_string(text: &str, lex: &mut Lex) -> Result<Json, String> {
    use self::Json::String as S;

    let mut escape = false;
    let mut res = String::new();
    let mut chars = (&text[lex.off..]).char_indices();

    let (i, ch) = chars.next().unwrap(); // skip the opening quote
    if ch != '"' {
        return Err(lex.format("parse: not a string"));
    }

    while let Some((i, ch)) = chars.next() {
        if !escape {
            if ch == '\\' {
                escape = true;
                continue;
            }
            match ch {
                '"' => {
                    lex.incr_col(i + 1);
                    return Ok(S(res));
                }
                _ => res.push(ch),
            }
            continue;
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
                code1 @ 0xDC00...0xDFFF => {
                    lex.incr_col(i);
                    let err = format!("parse: invalid char codepoint {}", code1);
                    return Err(lex.format(&err));
                }
                // Non-BMP characters are encoded as a sequence of
                // two hex escapes, representing UTF-16 surrogates.
                code1 @ 0xD800...0xDBFF => {
                    let code2 = decode_json_hex_code2(&mut chars, lex)?;
                    if code2 < 0xDC00 || code2 > 0xDFFF {
                        lex.incr_col(i);
                        let err = format!("parse: invalid codepoint {}", code2);
                        return Err(lex.format(&err));
                    }
                    let code = ((code1 - 0xD800) << 10) | ((code2 - 0xDC00) + 0x1_0000);
                    res.push(char::from_u32(code).unwrap());
                }

                n => match char::from_u32(n) {
                    Some(ch) => res.push(ch),
                    None => {
                        lex.incr_col(i);
                        let err = format!("parse: invalid escape code {:?}", n);
                        return Err(lex.format(&err));
                    }
                },
            },
            _ => {
                lex.incr_col(i);
                let err = "parse: invalid string string escape type";
                return Err(lex.format(&err));
            }
        }
        escape = false;
    }
    lex.incr_col(i);
    Err(lex.format("parse: incomplete string"))
}

fn decode_json_hex_code2(chars: &mut CharIndices, lex: &mut Lex) -> Result<u32, String> {
    if let Some((_, ch1)) = chars.next() {
        if let Some((_, ch2)) = chars.next() {
            if ch1 == '\\' && ch2 == 'u' {
                return decode_json_hex_code(chars, lex);
            }
        }
    }
    let err = "parse: invalid string string escape type";
    Err(lex.format(err))
}

fn decode_json_hex_code(chars: &mut CharIndices, lex: &mut Lex) -> Result<u32, String> {
    let mut n = 0;
    let mut code = 0_u32;
    for (_, ch) in chars {
        if (ch as u8) > 128 || HEXNUM[ch as usize] == 20 {
            let err = format!("parse: invalid string escape code {:?}", ch);
            return Err(lex.format(&err));
        }
        code = code * 16 + u32::from(HEXNUM[ch as usize]);
        n += 1;
        if n == 4 {
            break;
        }
    }
    if n != 4 {
        let err = format!("parse: incomplete string escape code {:x}", code);
        return Err(lex.format(&err));
    }
    Ok(code)
}

fn parse_array(text: &str, lex: &mut Lex) -> Result<Json, String> {
    lex.incr_col(1); // skip '['

    let mut array: Vec<Json> = Vec::new();
    parse_whitespace(text, lex);
    if (&text[lex.off..]).as_bytes()[0] == b',' {
        return Err(lex.format("parse: expected ','"));
    }
    loop {
        if (&text[lex.off..]).as_bytes()[0] == b']' {
            // end of array.
            lex.incr_col(1);
            break Ok(Json::Array(array));
        }

        array.push(parse_value(text, lex)?);

        parse_whitespace(text, lex);
        if (&text[lex.off..]).as_bytes()[0] == b',' {
            // skip comma
            lex.incr_col(1);
            parse_whitespace(text, lex);
        }
    }
}

fn parse_object(text: &str, lex: &mut Lex) -> Result<Json, String> {
    lex.incr_col(1); // skip '{'

    parse_whitespace(text, lex);

    let mut m: Vec<Property> = Vec::new();

    if (&text[lex.off..]).as_bytes()[0] == b'}' {
        lex.incr_col(1);
        return Ok(Json::Object(m));
    }

    loop {
        // key
        parse_whitespace(text, lex);
        let key: String = match text[lex.off..].chars().next() {
            Some('}') => {
                lex.incr_col(1);
                break Ok(Json::Object(m));
            }
            Some('"') => parse_string(text, lex)?.string().unwrap(),
            Some(ch) if ch.is_alphabetic() => parse_identifier(text, lex),
            _ => break Err(lex.format("parse: expected valid key")),
        };
        // colon
        parse_whitespace(text, lex);
        check_next_byte(text, lex, b':')?;

        // value
        parse_whitespace(text, lex);
        let value = parse_value(text, lex)?;

        property::upsert_object_key(&mut m, Property::new(key, value));
        //println!("parse {} {} {:?}", key, i, m);

        // is exit
        parse_whitespace(text, lex);
        let mut chars = text[lex.off..].chars();
        match chars.next() {
            None => break Err(lex.format("parse: unexpected eof")),
            Some(',') => {
                lex.incr_col(1);
            }
            _ => (),
        }
    }
}

fn parse_identifier(text: &str, lex: &mut Lex) -> String {
    let mut ident = String::new();
    for (i, ch) in text[lex.off..].char_indices() {
        if ch.is_alphanumeric() {
            ident.push(ch);
            continue;
        }
        lex.off += i;
        break;
    }
    ident
}

fn parse_whitespace(text: &str, lex: &mut Lex) {
    for (i, ch) in text[lex.off..].char_indices() {
        //println!("{} {}", u32::from(ch), ch.is_whitespace());
        if ch.is_whitespace() {
            if (u32::from(ch)) < 256 && ch == '\n' {
                lex.row += 1;
                lex.col = 0;
            } else {
                lex.col += 1;
            }
            continue;
        }
        lex.off += i;
        break;
    }
}

fn check_next_byte(text: &str, lex: &mut Lex, b: u8) -> Result<(), String> {
    let progbytes = (&text[lex.off..]).as_bytes();

    if progbytes.is_empty() {
        return Err(lex.format(&format!("parse: missing token {}", b)));
    }
    if progbytes[0] != b {
        return Err(lex.format(&format!("parse: invalid byte {}, {}", b, progbytes[0])));
    }
    lex.incr_col(1);

    Ok(())
}

fn check_eof(text: &str, lex: &mut Lex) -> Result<(), String> {
    if (&text[lex.off..]).is_empty() {
        Err(lex.format("parse: unexpected eof"))
    } else {
        Ok(())
    }
}

static HEXNUM: [u8; 256] = [
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 20, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 20, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 10, 11, 12, 13, 14, 15, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20,
];

static _WS_LOOKUP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static ISNUMBER: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 2, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
