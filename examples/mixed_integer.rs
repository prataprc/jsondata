use jsondata::{Json, JsonSerialize};

use std::str::FromStr;

#[derive(Debug, Clone, JsonSerialize)]
struct U8 {
    a: u8,
}

#[derive(Debug, Clone, JsonSerialize)]
struct F32 {
    a: f32,
}

#[derive(Debug, Clone, JsonSerialize)]
struct F64 {
    a: f64,
}

#[derive(Debug, Clone, JsonSerialize)]
struct Data {
    a: (u8, u8),
    b: (i8, i8),
    c: (u16, u16),
    d: (i16, i16),
    e: (u32, u32),
    f: (i32, i32),
    g: (u64, u64),
    h: (i64, i64),
    i: (usize, usize),
    j: (isize, isize),
}

fn main() {
    let data1 = Data {
        a: (0, 255),
        b: (-128, 127),
        c: (0, 65535),
        d: (-32768, 32767),
        e: (0, 4294967295),
        f: (-2147483648, 2147483647),
        g: (0, 18446744073709551615),
        h: (-9223372036854775808, 9223372036854775807),
        i: (0, 18446744073709551615),
        j: (-9223372036854775808, 9223372036854775807),
    };

    let jval1: Json = data1.into();
    let text = jval1.to_string();
    let out = Data::try_from(Json::from_str(&text).unwrap());
    assert!(out.is_ok());
    println!("{:?}", out.unwrap());

    let text = r#"{"a": 256}"#;
    let out = U8::try_from(Json::from_str(&text).unwrap());
    assert!(out.is_err());

    let text = r#"{"a": 2.123456789}"#;
    let out = F32::try_from(Json::from_str(&text).unwrap());
    assert!(out.is_ok());
    println!("{:?}", out.clone().unwrap());
    assert!(out.unwrap().a == 2.123456789); // Note .000000089 is ignored by f32.

    let texts = [
        r#"{"a": 0}"#,
        r#"{"a": -128}"#,
        r#"{"a": 127}"#,
        r#"{"a": 255}"#,
        r#"{"a": -32768}"#,
        r#"{"a": 32767}"#,
        r#"{"a": 65535}"#,
        r#"{"a": -2147483648}"#,
        r#"{"a": 2147483647}"#,
        r#"{"a": 4294967295}"#,
        r#"{"a": -9223372036854775808}"#,
        r#"{"a": 9223372036854775807}"#,
        r#"{"a": 18446744073709551615}"#,
    ];
    for (i, text) in texts.iter().enumerate() {
        let out = F64::try_from(Json::from_str(&text).unwrap());
        match i {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => println!("{:?}", out.unwrap()),
            _ => assert!(out.is_err()),
        }
    }
}
