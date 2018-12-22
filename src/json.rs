use std::fmt::{self, Write, Display};
use std::default::Default;
use std::convert::From;
use std::str::FromStr;
use std::cmp::Ordering;

use kv::{self, Property};
use lex::Lex;
use parse::parse_value;

#[derive(Clone,Debug)]
pub struct IntText {
    len: usize,
    txt: [u8; 32],
    val: Option<i128>,
}

impl IntText {
    pub fn new(txt: &str) -> IntText {
        let mut res = IntText{len: txt.len(), txt: [0_u8; 32], val: None};
        res.txt[..txt.len()].as_mut().copy_from_slice(txt.as_bytes());
        res
    }

    fn integer(&self) -> Option<i128> {
        use std::str::from_utf8;
        if self.val.is_none() {
            from_utf8(&self.txt[0..self.len]).unwrap().parse::<i128>().ok()
        } else {
            self.val
        }
    }

    fn compute(&mut self) {
        use std::str::from_utf8;
        if self.val.is_none() {
            self.val = from_utf8(&self.txt[0..self.len]).unwrap().parse::<i128>().ok();
        }
    }
}

impl Eq for IntText {}

impl PartialEq for IntText {
    fn eq(&self, other: &IntText) -> bool {
        self.integer() == other.integer()
    }
}

impl PartialOrd for IntText {
    fn partial_cmp(&self, other: &IntText) -> Option<Ordering> {
        self.integer().partial_cmp(&other.integer())
    }
}


#[derive(Clone,Debug)]
pub struct FloatText {
    len: usize,
    txt: [u8; 32],
    val: Option<f64>,
}

impl FloatText {
    pub fn new(txt: &str) -> FloatText {
        let mut res = FloatText{len: txt.len(), txt: [0_u8; 32], val: None};
        res.txt[..txt.len()].as_mut().copy_from_slice(txt.as_bytes());
        res
    }

    fn float(&self) -> Option<f64> {
        use std::str::from_utf8;
        if self.val.is_none() {
            from_utf8(&self.txt[0..self.len]).unwrap().parse::<f64>().ok()
        } else {
            self.val
        }
    }

    fn compute(&mut self) {
        use std::str::from_utf8;
        if self.val.is_none() {
            self.val = from_utf8(&self.txt[0..self.len]).unwrap().parse::<f64>().ok();
        }
    }
}

impl Eq for FloatText {}

impl PartialEq for FloatText {
    fn eq(&self, other: &FloatText) -> bool {
        self.float() == other.float()
    }
}

impl PartialOrd for FloatText {
    fn partial_cmp(&self, other: &FloatText) -> Option<Ordering> {
        self.float().partial_cmp(&other.float())
    }
}


// Json as rust native values.
#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(IntText),
    Float(FloatText),
    String(String),
    Array(Vec<Json>),
    Object(Vec<Property>),
    // TODO: Add error as a variant, that will help seamless
    // integrate with ops' dynamic errors.
}

impl Json {
    pub fn new<T>(value: T) -> Json where Self : From<T> {
        value.into()
    }

    pub fn boolean(&self) -> Option<bool> {
        match self { Json::Bool(s) => Some(*s), _ => None }
    }

    pub fn string(&self) -> Option<String> {
        match self { Json::String(s) => Some(s.clone()), _ => None }
    }

    pub fn integer(&self) -> Option<i128> {
        match self {
            Json::Integer(item) => item.integer(),
            _ => None
        }
    }

    pub fn float(&self) -> Option<f64> {
        match self {
            Json::Float(item) => item.float(),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<Vec<Json>> {
        match self { Json::Array(arr) => Some(arr.clone()), _ => None }
    }

    pub fn object(&self) -> Option<Vec<Property>> {
        match self { Json::Object(obj) => Some(obj.clone()), _ => None }
    }

    pub fn validate(&mut self) {
        use json::Json::{Array, Object, Integer, Float};

        match self {
            Array(arr) => arr.iter_mut().for_each(|v| v.validate()),
            Object(items) => items.iter_mut().for_each(|kv| kv.value_mut().validate()),
            Integer(item) => { item.compute(); },
            Float(item) => { item.compute(); },
            _ => (),
        };
    }

    pub fn compute(&mut self) {
        use json::Json::{Array, Object, Integer, Float};

        match self {
            Array(arr) => arr.iter_mut().for_each(|v| v.compute()),
            Object(items) => items.iter_mut().for_each(|kv| kv.value_mut().compute()),
            Integer(item) => { item.compute(); },
            Float(item) => { item.compute(); },
            _ => (),
        };
    }
}

impl Json {
    // TODO: should we expose this in rustdoc ?
    pub fn insert(&mut self, item: Property) {
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
        Json::Integer(IntText{len: 0, txt: [0_u8; 32], val: Some(val)})
    }
}

impl From<f64> for Json {
    fn from(val: f64) -> Json {
        Json::Float(FloatText{len: 0, txt: [0_u8; 32], val: Some(val)})
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

impl From<Vec<Property>> for Json {
    fn from(val: Vec<Property>) -> Json {
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
        use std::str::from_utf8;

        match self {
            Null => write!(f, "null"),
            Bool(true) => write!(f, "true"),
            Bool(false) => write!(f, "false"),
            Integer(IntText{len:_, txt:_, val: Some(v)}) => write!(f, "{}", v),
            Integer(IntText{len, txt, val:_}) => write!(f, "{}", from_utf8(&txt[..*len]).unwrap()),
            Float(FloatText{len:_, txt:_, val: Some(v)}) => write!(f, "{:e}", v),
            Float(FloatText{len, txt, val:_}) => write!(f, "{}", from_utf8(&txt[..*len]).unwrap()),
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
