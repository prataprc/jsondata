use std::{self, slice, vec, result, char, error, io};
use std::str::{self, FromStr,CharIndices};
use std::fmt::{self, Write};
use std::ops::{Neg, Not, Mul, Div, Rem, Add, Sub, Shr, Shl};
use std::ops::{BitAnd, BitXor, BitOr};

extern crate bnc_query as query;

use db::{self, Document, Doctype, Docindex};
use db::{And, Or, Slice, ItemIterator};
use lex::Lex;
use util;

include!("./json.rs.lookup");

#[derive(Clone,PartialEq,PartialOrd)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<Property>),
}

pub type Result<T> = result::Result<T,Error>;

impl Document for Json {
    type Err=Error;

    fn doctype(&self) -> Doctype {
        match self {
            Json::Null => Doctype::Null,
            Json::Bool(_) => Doctype::Bool,
            Json::Integer(_) => Doctype::Integer,
            Json::Float(_) => Doctype::Float,
            Json::String(_) => Doctype::String,
            Json::Array(_) => Doctype::Array,
            Json::Object(_) => Doctype::Object,
        }
    }

    fn len(self) -> Option<usize> {
        match self {
            Json::String(s) => Some(s.len()),
            Json::Array(a) => Some(a.len()),
            Json::Object(o) => Some(o.len()),
            Json::Null => Some(0),
            _ => None,
        }
    }

    fn set(&mut self, key: &str, value: Json) {
        match self {
            Json::Object(obj) => {
                match search_by_key(obj, key).ok() {
                    Ok(off) => obj[off].set_value(value),
                    Err(off) => obj[off].set_value(value),
                };
            },
            _ => panic!("cannot set {:?} with {}", self.doctype(), key),
        }
    }
}

impl Docindex<isize> for Json {
    fn index(self, off: isize) -> Option<Json> {
        match self {
            Json::Array(a) => {
                Some(a[util::normalized_offset(off, a.len())?].clone())
            }
            _ => None
        }
    }

    fn index_ref(&self, off: isize) -> Option<&Json> {
        match self {
            Json::Array(a) => {
                Some(&a[util::normalized_offset(off, a.len())?])
            }
            _ => None
        }
    }

    fn index_mut(&mut self, off: isize) -> Option<&mut Json> {
        match self {
            Json::Array(a) => {
                let off = util::normalized_offset(off, a.len())?;
                Some(&mut a[off])
            },
            _ => None
        }
    }

    fn get<'a>(self, key: &'a str) -> Option<Json> {
        match self {
            Json::Object(mut obj) => {
                let off = search_by_key(&obj, key).ok()?;
                Some(obj.remove(off).value())
            },
            _ => None
        }
    }

    fn get_ref<'a>(&self, key: &'a str) -> Option<&Json> {
        match self {
            Json::Object(obj) => {
                let off = search_by_key(obj, key).ok()?;
                Some(obj[off].value_ref())
            }
            _ => None,
        }
    }

    fn get_mut<'a>(&mut self, key: &'a str) -> Option<&mut Json> {
        match self {
            Json::Object(obj) => {
                let off = search_by_key(obj, key).ok()?;
                Some(obj[off].value_mut())
            }
            _ => None,
        }
    }
}

impl ItemIterator<Json> for Json {
    fn iter(&self) -> Option<slice::Iter<Json>> {
        match self {
            Json::Array(arr) => Some(arr.iter()),
            _ => None
        }
    }

    fn into_iter(self) -> Option<vec::IntoIter<Json>> {
        match self {
            Json::String(s) => {
                let out: Vec<Json> = s.chars().into_iter()
                    .map(|x| Json::Integer(x as i128))
                    .collect();
                Some(out.into_iter())
            },
            Json::Array(arr) => Some(arr.into_iter()),
            _ => None
        }
    }
}

impl ItemIterator<Property> for Json {
    fn iter(&self) -> Option<slice::Iter<Property>> {
        match self {
            Json::Object(obj) => Some(obj.iter()),
            _ => None
        }
    }

    fn into_iter(self) -> Option<vec::IntoIter<Property>> {
        match self {
            Json::Object(obj) => Some(obj.into_iter()),
            _ => None
        }
    }
}

impl Slice for Json {
    fn slice(self, start: isize, end: isize) -> Option<Json> {
        match self {
            Json::Array(arr) => {
                let (a, z) = util::slice_range_check(start, end, arr.len())?;
                Some(Json::Array(arr[a..z].to_vec()))
            },
            Json::String(s) => {
                let (a, z) = util::slice_range_check(start, end, s.len())?;
                Some(Json::String(s[a..z].to_string()))
            },
            _ => None,
        }
    }
}

impl Append<&str> for Json {
    fn append(&mut self, value: &str) {
        match self {
            Json::String(s) => s.push_str(value),
            _ => panic!("cannot append to {:?}", self.doctype()),
        }
    }
}

impl Append<Vec<Json>> for Json {
    fn append(&mut self, values: Vec<Json>) {
        match self {
            Json::Array(arr) => {
                values.iter().for_each(|val| arr.push(val.clone()))
            },
            _ => panic!("cannot append to {:?}", self.doctype()),
        }
    }
}

impl Append<Vec<Property>> for Json {
    fn append(&mut self, properties: Vec<Property>) {
        match self {
            Json::Object(obj) => {
                properties.into_iter().for_each(|prop| obj.insert(prop))
            }
            _ => panic!("cannot append to {:?}", self.doctype()),
        }
    }
}

//fn search_by_key(obj: &Vec<Property>, key: &str) -> Result<usize> {
//    match db::search_by_key(obj, key) {
//        Ok(off) => Ok(off),
//        Err(off) => Err(Error::KeyMissing(off, key.to_string())),
//    }
//}

#[cfg(test)] mod json_test;
