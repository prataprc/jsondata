use std::fmt::{self, Write, Display};
use std::default::Default;
use std::convert::From;
use std::str::FromStr;

use kv::{self, KeyValue};
use lex::Lex;
use parse::parse_value;

// Json as rust native values.
#[derive(Clone,PartialEq,PartialOrd)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<KeyValue>),
    // TODO: Add error as a variant, that will help seamless
    // integrate with ops' dynamic errors.
}

impl Json {
    pub fn new<T>(value: T) -> Json where Self : From<T> {
        value.into()
    }

    pub fn boolean(self) -> Option<bool> {
        match self { Json::Bool(s) => Some(s), _ => None }
    }

    pub fn string(self) -> Option<String> {
        match self { Json::String(s) => Some(s), _ => None }
    }

    pub fn integer(self) -> Option<i128> {
        match self { Json::Integer(n) => Some(n), _ => None }
    }

    pub fn float(self) -> Option<f64> {
        match self { Json::Float(f) => Some(f), _ => None }
    }

    pub fn array(self) -> Option<Vec<Json>> {
        match self { Json::Array(arr) => Some(arr), _ => None }
    }

    pub fn object(self) -> Option<Vec<KeyValue>> {
        match self { Json::Object(obj) => Some(obj), _ => None }
    }
}

impl Json {
    // TODO: should we expose this in rustdoc ?
    pub fn insert(&mut self, item: KeyValue) {
        match self {
            Json::Object(obj) => {
                match kv::search_by_key(obj, item.key_ref()) {
                    Ok(off) => obj.insert(off, item),
                    Err(off) => obj.insert(off, item),
                }
            },
            _ => ()
        }
    }
}

impl Default for Json {
    fn default() -> Json {
        Json::Null
    }
}

impl From<bool> for Json {
    fn from(val: bool) -> Json {
        Json::Bool(val)
    }
}

impl From<i128> for Json {
    fn from(val: i128) -> Json {
        Json::Integer(val)
    }
}

impl From<f64> for Json {
    fn from(val: f64) -> Json {
        Json::Float(val)
    }
}

impl From<String> for Json {
    fn from(val: String) -> Json {
        Json::String(val)
    }
}

impl From<&str> for Json {
    fn from(val: &str) -> Json {
        Json::String(val.to_string())
    }
}

impl From<Vec<Json>> for Json {
    fn from(val: Vec<Json>) -> Json {
        Json::Array(val)
    }
}

impl From<Vec<KeyValue>> for Json {
    fn from(val: Vec<KeyValue>) -> Json {
        Json::Object(val)
    }
}

impl From<Json> for bool {
    fn from(val: Json) -> bool {
        match val { Json::Null | Json::Bool(false) => false, _ => true }
    }
}

impl FromStr for Json {
    type Err=String;

    fn from_str(text: &str) -> Result<Json,String> {
        let mut lex = Lex::new(0, 1, 1);
        parse_value(&text, &mut lex)
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Json::{Null,Bool,Integer,Float,Array,Object, String as S};

        match self {
            Null => write!(f, "null"),
            Bool(true) => write!(f, "true"),
            Bool(false) => write!(f, "false"),
            Integer(val) => write!(f, "{}", val),
            Float(val) => write!(f, "{:e}", val),
            S(val) => { encode_string(f, &val)?; Ok(()) },
            Array(val) => {
                if val.len() == 0 {
                    write!(f, "[]")

                } else {
                    write!(f, "[")?;
                    for item in val[..val.len()-1].iter() {
                        write!(f, "{},", item)?;
                    }
                    write!(f, "{}", val[val.len()-1])?;
                    write!(f, "]")
                }
            },
            Object(val) => {
                let val_len = val.len();
                if val_len == 0 {
                    write!(f, "{{}}")

                } else {
                    write!(f, "{{")?;
                    for (i, kv) in val.iter().enumerate() {
                        encode_string(f, kv.key_ref())?;
                        write!(f, ":{}", kv.value_ref())?;
                        if i < (val_len - 1) { write!(f, ",")?; }
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}

impl fmt::Debug for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

fn encode_string<W: Write>(w: &mut W, val: &str) -> fmt::Result {
    write!(w, "\"")?;

    let mut start = 0;
    for (i, byte) in val.bytes().enumerate() {
        let escstr = ESCAPE[byte as usize];
        if escstr.len() == 0 { continue }

        if start < i {
            write!(w, "{}", &val[start..i])?;
        }
        write!(w, "{}", escstr)?;
        start = i + 1;
    }
    if start != val.len() {
        write!(w, "{}", &val[start..])?;
    }
    write!(w, "\"")
}


static ESCAPE: [&'static str; 256] = [
    "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004",
    "\\u0005", "\\u0006", "\\u0007", "\\b",     "\\t",
    "\\n",     "\\u000b", "\\f",     "\\r",     "\\u000e",
    "\\u000f", "\\u0010", "\\u0011", "\\u0012", "\\u0013",
    "\\u0014", "\\u0015", "\\u0016", "\\u0017", "\\u0018",
    "\\u0019", "\\u001a", "\\u001b", "\\u001c", "\\u001d",
    "\\u001e", "\\u001f", "",        "",        "\\\"",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "\\\\",    "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "\\u007f", "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",        "",        "",        "",        "",
    "",
];
