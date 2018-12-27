use std::fmt::{self, Write, Display};
use std::default::Default;
use std::convert::From;
use std::str::FromStr;

use property::{self, Property};
use lex::Lex;
use parse::parse_value;
use num::{Integral, Floating};
use jptr;

/// Json type implements JavaScript Object Notation as per specification
/// [RFC-8259](https://tools.ietf.org/html/rfc8259).
///
/// * Numbers are implemented with deferred conversion, using
///   ``Integral`` and ``Floating`` types.
/// * Arrays are implemented as vector of Json values.
/// * Objects are implemented as vector of properties, where each property
///   is a tuple of (key, value). Here key is [String] type and value is
///   Json type.
///
/// To parse JSON text, use [parse]:
///
/// ```
/// extern crate jsondata;
/// use jsondata::Json;
///
/// let text = r#"[null,true,false,10,"true"]"#;
/// let json = text.parse::<Json>(); // returns Result<Json,String>
/// ```
///
/// To serialise Json type to JSON text:
///
/// ```
/// extern crate jsondata;
/// use jsondata::Json;
///
/// let text = r#"[null,true,false,10,"true"]"#;
/// let json = text.parse::<Json>().unwrap();
///
/// let text1 = json.to_string();
/// let text2 = format!("{}", json);
/// assert_eq!(text1, text2);
/// ```
///
/// [string]: std::string::String
/// [parse]: str::method.parse
#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(Integral),
    Float(Floating),
    String(String),
    Array(Vec<Json>),
    Object(Vec<Property>),
}

/// Implementation provides methods to construct Json values.
impl Json {
    /// Construct [Json] from [bool], [i128], [f64], [String], [str],
    /// [Vec].
    ///
    /// Array can be composed as:
    ///
    /// ```
    /// extern crate jsondata;
    /// use jsondata::Json;
    ///
    /// let mut js = Json::new::<Vec<Json>>(Vec::new());
    /// js.append(Json::new(10));
    /// js.append(Json::new("hello world".to_string()));
    /// ```
    ///
    /// It is also possbile to construct the vector of Json outside
    /// the append() method, and finally use Json::new() to construct
    /// the array.
    ///
    /// Object can be composed as:
    ///
    /// ```
    /// extern crate jsondata;
    /// use jsondata::{Json, Property};
    ///
    /// let mut js = Json::new::<Vec<Property>>(Vec::new());
    /// js.insert(Property::new("key1".to_string(), Json::new(10)));
    /// js.insert(Property::new("key2".to_string(), Json::new(true)));
    /// ```
    ///
    /// It is also possbile to construct the vector of properties outside
    /// the insert() method, and finally use Json::new() to construct
    /// the object.
    pub fn new<T>(value: T) -> Json where Self : From<T> {
        value.into()
    }

    /// Validate parts of JSON text that are not yet parsed. Typically,
    /// when used in database context, JSON documents are validated once
    /// but parsed multiple times.
    pub fn validate(&mut self) -> Result<(), String> {
        use json::Json::{Array, Object, Integer, Float};

        match self {
            Array(items) => {
                for item in items.iter_mut() {
                    item.validate()?
                }
            }
            Object(props) => {
                for prop in props.iter_mut() {
                    prop.value_mut().validate()?
                }
            },
            Integer(item) => { item.compute()?; },
            Float(item) => { item.compute()?; },
            _ => (),
        };
        Ok(())
    }

    /// Compute parses unparsed text and convert them into numbers.
    /// When a JSON document is parsed once but operated on multiple
    /// times it is better to call compute for better performance.
    ///
    /// ```
    /// extern crate jsondata;
    /// use jsondata::Json;
    ///
    /// let text = r#"[null,true,false,10,"true"]"#;
    /// let mut json: Json = text.parse().unwrap();
    /// json.compute();
    ///
    /// // perform lookup and arithmetic operations on parsed document.
    /// ```
    pub fn compute(&mut self) -> Result<(), String> {
        use json::Json::{Array, Object, Integer, Float};

        match self {
            Array(items) => {
                for item in items.iter_mut() {
                    item.compute()?
                }
            },
            Object(props) => {
                for prop in props.iter_mut() {
                    prop.value_mut().compute()?
                }
            },
            Integer(item) => { item.compute()?; },
            Float(item) => { item.compute()?; },
            _ => (),
        };
        Ok(())
    }

    fn typename(&self) -> String {
        match self {
            Json::Null => "null".to_string(),
            Json::Bool(_) => "bool".to_string(),
            Json::Integer(_) => "integer".to_string(),
            Json::Float(_) => "float".to_string(),
            Json::String(_) => "string".to_string(),
            Json::Array(_) => "array".to_string(),
            Json::Object(_) => "object".to_string(),
        }
    }
}

impl Json {
    pub fn get(&self, jptr: &str) -> Result<Json,String> {
        if jptr.len() == 0 {
            Ok(self.clone())

        } else {
            let (json, frag) = jptr::g_lookup(self, jptr)?;
            let json = jptr::g_lookup_container(json, &frag)?;
            Ok(json.clone())
        }
    }

    pub fn set(&mut self, jptr: &str, value: Json) -> Result<(),String> {

        if jptr.len() == 0 { return Ok(()) }

        let (json, frag) = jptr::lookup(self, jptr)?;
        match json {
            Json::Array(arr) => {
                match frag.parse::<usize>() {
                    Ok(n) if n >= arr.len() => Err(format!("jptr: index out of bound {}", n)),
                    Ok(n) => { arr[n] = value; Ok(()) },
                    Err(err) => Err(format!("jptr: not array-index {}", err)),
                }
            },
            Json::Object(props) => {
                match property::search_by_key(&props, &frag) {
                    Ok(n) => Ok(props.insert(n, Property::new(frag, value))),
                    Err(n) => Ok(props.insert(n, Property::new(frag, value))),
                }
            },
            _ => Err(format!("jptr: not a container {} at {}", json, frag)),
        }
    }

    pub fn delete(&mut self, jptr: &str) -> Result<(),String> {
        if jptr.len() == 0 { return Ok(()) }

        let (json, frag) = jptr::lookup(self, jptr)?;
        match json {
            Json::Array(arr) => {
                match frag.parse::<usize>() {
                    Ok(n) if n >= arr.len() => Err(format!("jptr: index out of bound {}", n)),
                    Ok(n) => {arr.remove(n); Ok(())},
                    Err(err) => Err(format!("jptr: not array-index {}", err)),
                }
            },
            Json::Object(props) => {
                match property::search_by_key(&props, &frag) {
                    Ok(n) => {props.remove(n); Ok(())},
                    Err(_) => Err(format!("jptr: key {} not found", frag)),
                }
            },
            _ => Err(format!("{} not a container type", json.typename())),
        }
    }

    pub fn append(&mut self, jptr: &str, value: Json ) -> Result<(), String> {

        if jptr.len() == 0 { return Ok(()) }

        let (json, frag) = jptr::lookup(self, jptr)?;
        let json = jptr::lookup_container(json, &frag)?;
        match json {
            Json::String(s1) => {
                if let Json::String(s2) = value {
                    let mut s = String::new();
                    s.push_str(&s1); s.push_str(&s2);
                    Ok(())
                } else {
                    Err(format!("jptr: cannot add {} to `{}`", value.typename(), s1))
                }
            },
            Json::Array(arr) => { let n = arr.len(); Ok(arr.insert(n, value)) },
            _ => Err(format!("jptr: not a container {} at {}", json, frag)),
        }
    }
}

/// Implementation clones underlying type for each Json variant.
/// The return value is always an [Option] because JSON
/// follows a schemaless data representation.
impl Json {
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
}

impl Json {
    // TODO: should we expose this in rustdoc ?
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
        Json::Integer(Integral::new(val))
    }
}

impl From<f64> for Json {
    fn from(val: f64) -> Json {
        Json::Float(Floating::new(val))
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

    fn from_str(text: &str) -> Result<Json, String> {
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
            Integer(Integral{len:_, txt:_, val: Some(v)}) => write!(f, "{}", v),
            Integer(Integral{len, txt, val:_}) => write!(f, "{}", from_utf8(&txt[..*len]).unwrap()),
            Float(Floating{len:_, txt:_, val: Some(v)}) => write!(f, "{:e}", v),
            Float(Floating{len, txt, val:_}) => write!(f, "{}", from_utf8(&txt[..*len]).unwrap()),
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
                    for (i, prop) in val.iter().enumerate() {
                        encode_string(f, prop.key_ref())?;
                        write!(f, ":{}", prop.value_ref())?;
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

pub fn insert(json: &mut Json, item: Property) {
    match json {
        Json::Object(obj) => {
            match property::search_by_key(&obj, item.key_ref()) {
                Ok(off) => obj.insert(off, item),
                Err(off) => obj.insert(off, item),
            }
        },
        _ => ()
    }
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
