// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::{From, TryFrom, TryInto};
use std::default::Default;
use std::fmt::{self, Display, Write};
use std::ops::RangeBounds;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::lex::Lex;
use crate::num::{Floating, Integral};
use crate::parse::parse_value;
use crate::property::{self, Property};
use crate::{jptr, ops};

/// Json type implements JavaScript Object Notation as per specification
/// [RFC-8259](https://tools.ietf.org/html/rfc8259).
///
/// * JSON scalar types - Null, Number, Boolean, String, are supported.
/// * JSON container types - Array, Object, are supported.
/// * JSON numbers can be 128-bit integers or 64-bit floating point.
/// * When document is known to contain lot of numbers and only one of
///   them needs to be extracted, parsing the entire document can be
///   inefficient just to get that one field. Numbers are implemented
///   with deferred conversion, using ``Integral`` and ``Floating`` types.
/// * Arrays are implemented as vector of Json values Vec<[Json]>.
/// * Objects are implemented as vector of properties, Vec<[Property]>,
///   where each property is a tuple of (key, value). Here key is [String]
///   type and value is [Json] type.
///
/// [Json] enum type has documented variants and undocumented variants.
/// Applications, when matching with Json, must use the catch-all variant:
/// ```ignore
/// match json {
///     Json::Null => // handle null,
///     Json::Bool(b) => // handle bool,
///     Json::Integer(i) => // handle integer,
///     Json::Float(f) => // handle float,
///     Json::String(s) => // handle string,
///     Json::Array(a) => // handle array,
///     Json::Object(o) => // handle object,
///     _ => // catch all.
/// }
/// ```
///
///
/// **Parsing JSON text**:
/// ```
/// let json: jsondata::Json = "10".parse().unwrap();
/// ```
///
/// return a [Json] enum type.
///
/// **Converting Rust native types to [Json] enum**:
///
/// Json supports conversion from [bool], [i128], [f64], [String], &str,
/// Vec<[Json]> and Vec<[Property]> types using the [From] trait.
/// ```
/// let json: jsondata::Json = 10.into();
/// let json: jsondata::Json = true.into();
/// let json: jsondata::Json = "hello world".into();
/// ```
///
/// On the other direction, [Json] enum can be converted to Rust native
/// types using accessor methods,
/// - is_null() to check whether [Json] is Null
/// - boolean(), integer(), float(), string() methods return the
///   underlying value as Option<`T`> where `T` is [bool] or [i128] or [f64] or
///   [String].
/// - array(), return JSON array as Vec<[Json]>.
/// - object(), return JSON object as Vec<[Property]>.
///
/// Some of the properties implemented for [Json] are:
/// - [Json] implements [total ordering].
/// - Default value for Json is Null.
/// - Json types are clone-able but do not implement [Copy].
/// - [Json] value can be serialized into JSON format using [Display] trait.
///
/// **Panics**
///
/// [Json] implements AsRef and AsMut traits for [str], Vec<[Json]>,
/// Vec<[Property]> types. This means, call to as_ref() and as_mut() shall
/// panic when underlying Json variant do not match with expected type.
///
/// [string]: std::string::String
/// [total ordering]: https://en.wikipedia.org/wiki/Total_order
#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(Integral),
    Float(Floating),
    String(String),
    Array(Vec<Json>),
    Object(Vec<Property>),
    // Hidden variants
    #[doc(hidden)]
    __Error(Error),
    #[doc(hidden)]
    __Minbound,
    #[doc(hidden)]
    __Maxbound,
}

/// Implementation provides methods to construct and validate Json values.
impl Json {
    /// Construct [Json] from [bool], [i128], [f64], [String], [str],
    /// [Vec].
    ///
    /// Array can be composed as:
    ///
    /// ```
    /// use jsondata::Json;
    ///
    /// let mut js = Json::new::<Vec<Json>>(Vec::new());
    /// js.append("", Json::new(10));
    /// js.append("", Json::new("hello world".to_string()));
    /// ```
    ///
    /// It is also possible to construct the vector of Json outside
    /// the append() method, and finally use Json::new() to construct
    /// the array.
    ///
    /// Object can be composed as:
    ///
    /// ```
    /// use jsondata::{Json, Property};
    ///
    /// let mut js = Json::new::<Vec<Property>>(Vec::new());
    /// js.set("/key1", Json::new(10));
    /// js.set("/key2", Json::new(true));
    /// ```
    ///
    /// It is also possible to construct the vector of properties outside
    /// the set() method, and finally use Json::new() to construct
    /// the object.
    pub fn new<T>(value: T) -> Json
    where
        Self: From<T>,
    {
        value.into()
    }

    /// Minbound return a Json value that sort before every other [Json] type.
    #[allow(dead_code)]
    pub(crate) fn minbound() -> Json {
        Json::__Minbound
    }

    /// Maxbound return a Json value that sort after every other [Json] type.
    #[allow(dead_code)]
    pub(crate) fn maxbound() -> Json {
        Json::__Maxbound
    }

    /// Validate parts of JSON text that are not yet parsed. Typically,
    /// when used in database context, JSON documents are validated once
    /// but parsed multiple times.
    pub fn validate(&mut self) -> Result<()> {
        use crate::json::Json::{Array, Float, Integer, Object};

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
            }
            Integer(item) => {
                item.compute()?;
            }
            Float(item) => {
                item.compute()?;
            }
            _ => (),
        };
        Ok(())
    }

    /// Compute parses unparsed text and convert them into numbers.
    /// When a JSON document is parsed once but operated on multiple
    /// times it is better to call compute for better performance.
    ///
    /// ```
    /// use jsondata::Json;
    ///
    /// let text = r#"[null,true,false,10,"true"]"#;
    /// let mut json: Json = text.parse().unwrap();
    /// json.compute();
    ///
    /// // perform lookup and arithmetic operations on parsed document.
    /// ```
    pub fn compute(&mut self) -> Result<()> {
        use crate::json::Json::{Array, Float, Integer, Object};

        match self {
            Array(items) => {
                for item in items.iter_mut() {
                    item.compute()?
                }
            }
            Object(props) => {
                for prop in props.iter_mut() {
                    prop.value_mut().compute()?
                }
            }
            Integer(item) => {
                item.compute()?;
            }
            Float(item) => {
                item.compute()?;
            }
            _ => (),
        };
        Ok(())
    }

    // TODO: rename this to type_name()
    pub fn typename(&self) -> String {
        match self {
            Json::Null => "null".to_string(),
            Json::Bool(_) => "bool".to_string(),
            Json::Integer(_) => "integer".to_string(),
            Json::Float(_) => "float".to_string(),
            Json::String(_) => "string".to_string(),
            Json::Array(_) => "array".to_string(),
            Json::Object(_) => "object".to_string(),
            Json::__Error(_) => "error".to_string(),
            Json::__Minbound => "minbound".to_string(),
            Json::__Maxbound => "maxbound".to_string(),
        }
    }
}

/// Implementation provides CRUD access into [Json] document using
/// [Json Pointer]. For all methods,
///
/// * Path must be valid JSON Pointer.
/// * Path fragment must be valid key if parent container is an object.
/// * Path fragment must be a number index if parent container is an array.
///
/// [JSON Pointer]: https://tools.ietf.org/html/rfc6901
impl Json {
    /// Get a json field, within the document, locatable by ``path``.
    pub fn get(&self, path: &str) -> Result<Json> {
        if path.is_empty() {
            Ok(self.clone())
        } else {
            let path = jptr::fix_prefix(path)?;
            let (json, key) = jptr::lookup_ref(self, path)?;
            Ok(json[key.as_str()].result()?.clone())
        }
    }

    /// Set a json field, within the document, locatable by ``path``.
    pub fn set(&mut self, path: &str, value: Json) -> Result<()> {
        if path.is_empty() {
            return Ok(());
        }

        let path = jptr::fix_prefix(path)?;

        let (json, frag) = jptr::lookup_mut(self, path)?;
        match json {
            Json::Array(arr) => match frag.parse::<usize>() {
                Ok(n) => {
                    if n >= arr.len() {
                        Err(Error::IndexOutofBound(n.try_into().unwrap()))
                    } else {
                        arr[n] = value;
                        Ok(())
                    }
                }
                Err(err) => Err(Error::InvalidIndex(err.to_string())),
            },
            Json::Object(props) => match property::search_by_key(&props, &frag) {
                Ok(n) => {
                    props[n].set_value(value);
                    Ok(())
                }
                Err(n) => {
                    props.insert(n, Property::new(frag, value));
                    Ok(())
                }
            },
            _ => Err(Error::InvalidContainer(json.typename())),
        }
    }

    /// Delete a JSON field, within the document, locatable by ``path``.
    pub fn delete(&mut self, path: &str) -> Result<()> {
        if path.is_empty() {
            return Ok(());
        }

        let path = jptr::fix_prefix(path)?;

        let (json, frag) = jptr::lookup_mut(self, path)?;
        match json {
            Json::Array(arr) => match frag.parse::<usize>() {
                Ok(n) => {
                    if n >= arr.len() {
                        Err(Error::IndexOutofBound(n.try_into().unwrap()))
                    } else {
                        arr.remove(n);
                        Ok(())
                    }
                }
                Err(err) => Err(Error::InvalidIndex(err.to_string())),
            },
            Json::Object(props) => match property::search_by_key(&props, &frag) {
                Ok(n) => {
                    props.remove(n);
                    Ok(())
                }
                Err(_) => Err(Error::PropertyNotFound(frag)),
            },
            _ => Err(Error::InvalidContainer(json.typename())),
        }
    }

    /// Append a string or array to a JSON field within the document that is
    /// either a string or array.
    pub fn append(&mut self, path: &str, value: Json) -> Result<()> {
        if path.is_empty() {
            return Ok(());
        }
        let path = jptr::fix_prefix(path)?;

        let (json, frag) = jptr::lookup_mut(self, path)?;
        match ops::index_mut(json, frag.as_str())? {
            Json::String(j) => {
                if let Json::String(s) = value {
                    j.push_str(&s);
                    Ok(())
                } else {
                    Err(Error::AppendString(value.typename()))
                }
            }
            Json::Array(arr) => {
                let n = arr.len();
                arr.insert(n, value);
                Ok(())
            }
            _ => Err(Error::InvalidContainer(json.typename())),
        }
    }

    /// Range operation on Json array,
    ///
    /// * Range              ``[start..end]``.
    /// * RangeFrom          ``[start..]``.
    /// * RangeFull          ``[..]``.
    /// * RangeInclusive     ``[start..=end]``.
    /// * RangeTo            ``[..end]``.
    /// * RangeToInclusive   ``[..=end]``.
    ///
    /// If range is called on non array Json, returns a Json Error.
    pub fn range<R>(&self, range: R) -> Json
    where
        R: RangeBounds<isize>,
    {
        use std::ops::Bound::{Excluded, Included, Unbounded};

        match self {
            Json::__Error(_) => self.clone(),
            Json::Array(arr) => {
                let (start, s) = match range.start_bound() {
                    Included(n) => (ops::normalized_offset(*n, arr.len()), *n),
                    Excluded(n) => (ops::normalized_offset((*n) + 1, arr.len()), *n),
                    Unbounded => (Some(0), 0),
                };
                let (end, e) = match range.end_bound() {
                    Included(n) => (ops::normalized_offset((*n) + 1, arr.len()), *n),
                    Excluded(n) => (ops::normalized_offset(*n, arr.len()), *n),
                    Unbounded => (Some(arr.len()), arr.len().try_into().unwrap()),
                };
                match (start, end) {
                    (Some(start), Some(end)) => arr[start..end].to_vec().into(),
                    (None, _) => Json::__Error(Error::IndexOutofBound(s)),
                    (_, None) => Json::__Error(Error::IndexOutofBound(e)),
                }
            }
            _ => Json::__Error(Error::NotAnArray(format!("{}", self))),
        }
    }
}

/// Implementation clones underlying type for each Json variant.
/// The return value is always an [Option] because JSON
/// follows a schema-less data representation.
impl Json {
    pub fn is_null(&self) -> bool {
        match self {
            Json::Null => true,
            _ => false,
        }
    }

    pub fn boolean(&self) -> Option<bool> {
        match self {
            Json::Bool(s) => Some(*s),
            _ => None,
        }
    }

    pub fn integer(&self) -> Option<i128> {
        match self {
            Json::Integer(item) => item.integer(),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f64> {
        match self {
            Json::Float(item) => item.float(),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<String> {
        match self {
            Json::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<Vec<Json>> {
        match self {
            Json::Array(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    pub fn object(&self) -> Option<Vec<Property>> {
        match self {
            Json::Object(obj) => Some(obj.clone()),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Json::__Error(_) => true,
            _ => false,
        }
    }

    pub fn error(&self) -> Option<Error> {
        match self {
            Json::__Error(err) => Some(err.clone()),
            _ => None,
        }
    }

    pub fn result(&self) -> Result<&Json> {
        match self {
            Json::__Error(err) => Err(err.clone()),
            _ => Ok(self),
        }
    }
}

impl Eq for Json {}

impl PartialEq for Json {
    fn eq(&self, other: &Json) -> bool {
        use crate::Json::{Array, Bool, Float, Integer, Null, Object, String as S};
        use std::i128;

        match (self, other) {
            (Null, Null) => true,
            (Bool(a), Bool(b)) => a == b,
            (Integer(_), Integer(_)) => self.integer() == other.integer(),
            (Integer(a), Float(b)) => match (a.integer(), b.float()) {
                (Some(x), Some(y)) => {
                    let num = y as i128;
                    if num == i128::MIN || num == i128::MAX || y.is_nan() {
                        return false;
                    }
                    x == num
                }
                _ => false,
            },
            (Float(_), Float(_)) => {
                let (fs, fo) = (self.float().unwrap(), other.float().unwrap());
                if fs.is_finite() && fo.is_finite() {
                    return fs == fo;
                } else if fs.is_nan() && fo.is_nan() {
                    return true;
                } else if fs.is_infinite() && fo.is_infinite() {
                    return fs.is_sign_positive() == fo.is_sign_positive();
                }
                false
            }
            (Float(a), Integer(b)) => match (a.float(), b.integer()) {
                (Some(x), Some(y)) => {
                    let num = x as i128;
                    if num == i128::MIN || num == i128::MAX || x.is_nan() {
                        return false;
                    }
                    y == num
                }
                _ => false,
            },
            (S(a), S(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            (Object(a), Object(b)) => a == b,
            // handle boundaries
            (Json::__Minbound, Json::__Minbound) => true,
            (Json::__Maxbound, Json::__Maxbound) => true,
            // catch all
            _ => false,
        }
    }
}

impl PartialOrd for Json {
    fn partial_cmp(&self, other: &Json) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Json {
    fn cmp(&self, other: &Json) -> Ordering {
        use crate::Json::{Array, Bool, Float, Integer, Null, Object, String as S};

        match (self, other) {
            // typically we assume that value at same position is same type.
            (Null, Null) => Ordering::Equal,
            (Bool(a), Bool(b)) => {
                if (*a) == (*b) {
                    Ordering::Equal
                } else if !(*a) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Integer(a), Integer(b)) => {
                let (x, y) = (a.integer().unwrap(), b.integer().unwrap());
                x.cmp(&y)
            }
            (Float(a), Float(b)) => {
                let (fs, fo) = (a.float().unwrap(), b.float().unwrap());
                if fs.is_finite() && fo.is_finite() {
                    if fs < fo {
                        Ordering::Less
                    } else if fs > fo {
                        Ordering::Greater
                    } else if fs == 0.0 || fo == 0.0 {
                        if fs.is_sign_negative() {
                            Ordering::Less
                        } else if fo.is_sign_negative() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    } else {
                        Ordering::Equal
                    }
                } else {
                    let is = if fs.is_infinite() {
                        fs.signum() as i32
                    } else {
                        2
                    };
                    let io = if fo.is_infinite() {
                        fo.signum() as i32
                    } else {
                        2
                    };
                    is.cmp(&io)
                }
            }
            (Integer(a), Float(b)) => match (a.integer(), b.float()) {
                (Some(x), Some(y)) => x.cmp(&(y as i128)),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            (Float(a), Integer(b)) => match (a.float(), b.integer()) {
                (Some(x), Some(y)) => (x as i128).cmp(&y),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            (S(a), S(b)) => a.cmp(b),
            (Array(this), Array(that)) => {
                for (i, a) in this.iter().enumerate() {
                    if i == that.len() {
                        return Ordering::Greater;
                    }
                    let cmp = a.cmp(&that[i]);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                if this.len() == that.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            (Object(this), Object(that)) => {
                for (i, a) in this.iter().enumerate() {
                    if i == that.len() {
                        return Ordering::Greater;
                    }
                    let cmp = a.key_ref().cmp(that[i].key_ref());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                    let cmp = a.value_ref().cmp(that[i].value_ref());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                if this.len() == that.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            // handle error cases, error variants sort at the end.
            (_, Json::__Error(_)) => Ordering::Less,
            (Json::__Error(_), _) => Ordering::Greater,
            // handle boundaries
            (Json::__Minbound, Json::__Minbound) => Ordering::Equal,
            (Json::__Maxbound, Json::__Maxbound) => Ordering::Equal,
            (Json::__Minbound, _) => Ordering::Less,
            (Json::__Maxbound, _) => Ordering::Greater,
            (_, Json::__Minbound) => Ordering::Greater,
            (_, Json::__Maxbound) => Ordering::Less,
            // handle cases of mixed types.
            (Null, _) => Ordering::Less,
            (_, Null) => Ordering::Greater,
            (Bool(_), _) => Ordering::Less,
            (_, Bool(_)) => Ordering::Greater,
            (Integer(_), _) => Ordering::Less,
            (_, Integer(_)) => Ordering::Greater,
            (Float(_), _) => Ordering::Less,
            (_, Float(_)) => Ordering::Greater,
            (S(_), _) => Ordering::Less,
            (_, S(_)) => Ordering::Greater,
            (Array(_), _) => Ordering::Less,
            (_, Array(_)) => Ordering::Greater,
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
        Json::Integer(Integral::new(val))
    }
}

impl TryFrom<Json> for i128 {
    type Error = Error;

    fn try_from(val: Json) -> Result<i128> {
        match val {
            Json::Integer(item) => match item.integer() {
                Some(num) => Ok(num),
                None => Err(Error::InvalidNumber(format!("{:?}", item))),
            },
            _ => Err(Error::InvalidType(val.typename())),
        }
    }
}

impl From<usize> for Json {
    fn from(val: usize) -> Json {
        let val: i128 = val.try_into().unwrap();
        Json::Integer(Integral::new(val))
    }
}

impl From<u64> for Json {
    fn from(val: u64) -> Json {
        let val: i128 = val.try_into().unwrap();
        Json::Integer(Integral::new(val))
    }
}

impl From<i32> for Json {
    fn from(val: i32) -> Json {
        let val: i128 = val.try_into().unwrap();
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
        let mut obj = Json::Object(vec![]);
        val.into_iter().for_each(|item| insert(&mut obj, item));
        obj
    }
}

impl From<Json> for bool {
    fn from(val: Json) -> bool {
        use crate::json::Json::String as S;
        use crate::json::Json::{Array, Bool, Float, Integer, Null, Object};

        match val {
            Null => false,
            Bool(v) => v,
            Integer(_) => val.integer().unwrap() != 0,
            Float(_) => val.float().unwrap() != 0.0,
            S(s) => !s.is_empty(),
            Array(a) => !a.is_empty(),
            Object(o) => !o.is_empty(),
            Json::__Error(_) => false,
            Json::__Minbound => true,
            Json::__Maxbound => true,
        }
    }
}

impl FromStr for Json {
    type Err = Error;

    fn from_str(text: &str) -> Result<Json> {
        let mut lex = Lex::new(0, 1, 1);
        parse_value(&text, &mut lex)
    }
}

impl AsRef<str> for Json {
    fn as_ref(&self) -> &str {
        match self {
            Json::String(s) => s,
            _ => panic!("Json is not string"),
        }
    }
}

impl AsRef<Vec<Json>> for Json {
    fn as_ref(&self) -> &Vec<Json> {
        match self {
            Json::Array(arr) => arr,
            _ => panic!("Json is not an array"),
        }
    }
}

impl AsRef<Vec<Property>> for Json {
    fn as_ref(&self) -> &Vec<Property> {
        match self {
            Json::Object(obj) => obj,
            _ => panic!("Json is not an object"),
        }
    }
}

impl AsMut<str> for Json {
    fn as_mut(&mut self) -> &mut str {
        match self {
            Json::String(s) => s,
            _ => panic!("Json is not string"),
        }
    }
}

impl AsMut<Vec<Json>> for Json {
    fn as_mut(&mut self) -> &mut Vec<Json> {
        match self {
            Json::Array(arr) => arr,
            _ => panic!("Json is not an array"),
        }
    }
}

impl AsMut<Vec<Property>> for Json {
    fn as_mut(&mut self) -> &mut Vec<Property> {
        match self {
            Json::Object(obj) => obj,
            _ => panic!("Json is not an object"),
        }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Json::{Array, Bool, Float, Integer, Null, Object, String as S};
        use std::str::from_utf8;

        match self {
            Null => write!(f, "null"),
            Bool(true) => write!(f, "true"),
            Bool(false) => write!(f, "false"),
            Integer(Integral::Text { len, bytes }) => {
                let arg = from_utf8(&bytes[..*len]).unwrap();
                write!(f, "{}", arg)
            }
            Integer(Integral::Data { value: v }) => write!(f, "{}", v),
            Float(Floating::Text { len, bytes }) => {
                let arg = from_utf8(&bytes[..*len]).unwrap();
                write!(f, "{}", arg)
            }
            Float(Floating::Data { value: v }) => write!(f, "{:e}", v),
            S(val) => {
                encode_string(f, &val)?;
                Ok(())
            }
            Array(val) => {
                if val.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(f, "[")?;
                    for item in val[..val.len() - 1].iter() {
                        write!(f, "{},", item)?;
                    }
                    write!(f, "{}", val[val.len() - 1])?;
                    write!(f, "]")
                }
            }
            Object(val) => {
                let val_len = val.len();
                if val_len == 0 {
                    write!(f, "{{}}")
                } else {
                    write!(f, "{{")?;
                    for (i, prop) in val.iter().enumerate() {
                        encode_string(f, prop.key_ref())?;
                        write!(f, ":{}", prop.value_ref())?;
                        if i < (val_len - 1) {
                            write!(f, ",")?;
                        }
                    }
                    write!(f, "}}")
                }
            }
            Json::__Error(err) => write!(f, "error: {:?}", err),
            Json::__Minbound => write!(f, "minbound"),
            Json::__Maxbound => write!(f, "maxbound"),
        }
    }
}

fn encode_string<W: Write>(w: &mut W, val: &str) -> fmt::Result {
    write!(w, "\"")?;

    let mut start = 0;
    for (i, byte) in val.bytes().enumerate() {
        let escstr = ESCAPE[usize::from(byte)];
        if escstr.is_empty() {
            continue;
        }

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
    if let Json::Object(obj) = json {
        match property::search_by_key(&obj, item.key_ref()) {
            Ok(off) => {
                obj.push(item);
                obj.swap_remove(off);
            }
            Err(off) => obj.insert(off, item),
        }
    }
}

static ESCAPE: [&str; 256] = [
    "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004", "\\u0005", "\\u0006", "\\u0007", "\\b",
    "\\t", "\\n", "\\u000b", "\\f", "\\r", "\\u000e", "\\u000f", "\\u0010", "\\u0011", "\\u0012",
    "\\u0013", "\\u0014", "\\u0015", "\\u0016", "\\u0017", "\\u0018", "\\u0019", "\\u001a",
    "\\u001b", "\\u001c", "\\u001d", "\\u001e", "\\u001f", "", "", "\\\"", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "\\\\", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "\\u007f", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "",
];

#[cfg(test)]
#[path = "json_test.rs"]
mod json_test;
