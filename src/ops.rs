// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use std::ops::{Add, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Index};

use lazy_static::lazy_static;

use crate::{json::Json, property::Property, Error, Result};

// TODO: Implement && || as short-circuiting logical operation. They are not
// not implementable as `std` traits, hence figure out an apt API.

macro_rules! check_error {
    ($res:expr) => {{
        match $res {
            Ok(val) => val,
            Err(err) => return Json::__Error(err),
        }
    }};
}

impl Add for Json {
    type Output = Json;

    fn add(self, rhs: Json) -> Json {
        use crate::json::Json::{Array, Float, Integer, Null, Object, String as S};

        match (&self, &rhs) {
            (Null, _) => rhs.clone(),  // Identity operation
            (_, Null) => self.clone(), // Identity operation
            (Integer(_), Integer(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l + r)
            }
            (Float(_), Float(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_float_result());
                Json::new(l + r)
            }
            (Integer(_), Float(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_float_result());
                Json::new(l as f64 + r)
            }
            (Float(_), Integer(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l + r as f64)
            }
            (S(l), S(r)) => {
                let mut s = String::new();
                s.push_str(l);
                s.push_str(r);
                S(s)
            }
            (Array(l), Array(r)) => {
                let mut a = vec![];
                a.extend_from_slice(l);
                a.extend_from_slice(r);
                Array(a)
            }
            (Object(l), Object(r)) => {
                use crate::json;

                let mut obj = Json::Object(Vec::new());
                l.iter().cloned().for_each(|p| json::insert(&mut obj, p));
                r.iter().cloned().for_each(|p| json::insert(&mut obj, p));
                obj
            }
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(AddFail, msg: "{} + {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Sub for Json {
    type Output = Json;

    fn sub(self, rhs: Json) -> Json {
        use crate::json::Json::{Array, Float, Integer, Null, Object};

        match (&self, &rhs) {
            (Null, _) => rhs.clone(),  // Identity operation
            (_, Null) => self.clone(), // Identity operation
            (Integer(_), Integer(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l - r)
            }
            (Float(_), Float(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_float_result());
                Json::new(l - r)
            }
            (Integer(_), Float(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_float_result());
                Json::new((l as f64) - r)
            }
            (Float(_), Integer(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l - (r as f64))
            }
            (Array(lhs), Array(rhs)) => {
                let mut res = lhs.clone();
                rhs.iter().for_each(|x| {
                    if let Some(pos) = res.iter().position(|y| *x == *y) {
                        res.remove(pos);
                    }
                });
                Array(res)
            }
            (Object(lhs), Object(rhs)) => {
                let mut res = lhs.clone();
                rhs.iter().for_each(|x| {
                    if let Some(pos) = res.iter().position(|y| *x == *y) {
                        res.remove(pos);
                    }
                });
                Object(res)
            }
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(SubFail, msg: "{} - {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Mul for Json {
    type Output = Json;

    fn mul(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null, Object, String as S};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l * r)
            }
            (Float(_), Float(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_float_result());
                Json::new(l * r)
            }
            (Integer(_), Float(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_float_result());
                Json::new((l as f64) * r)
            }
            (Float(_), Integer(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_integer_result());
                Json::new(l * (r as f64))
            }
            (S(s), Integer(_)) => match rhs.to_integer_result() {
                Ok(n) if n <= 0 => Null,
                Ok(n) => S(s.repeat(n as usize)),
                Err(err) => Json::__Error(err),
            },
            (Integer(_), S(s)) => match self.to_integer_result() {
                Ok(n) if n <= 0 => Null,
                Ok(n) => S(s.repeat(n as usize)),
                Err(err) => Json::__Error(err),
            },
            (Object(this), Object(other)) => {
                // TODO: this is not well defined.
                let mut obj = Vec::new();
                obj = mixin_object(obj, this.to_vec());
                obj = mixin_object(obj, other.to_vec());
                Json::Object(obj)
            }
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(MulFail, msg: "{} * {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Div for Json {
    type Output = Json;

    fn div(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null, String as S};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_integer_result());
                if r == 0 {
                    Null
                } else {
                    Json::new(l / r)
                }
            }
            (Float(_), Float(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_float_result());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new(l / r)
                }
            }
            (Integer(_), Float(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_float_result());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new((l as f64) / r)
                }
            }
            (Float(_), Integer(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_integer_result());
                if r == 0 {
                    Null
                } else {
                    Json::new(l / (r as f64))
                }
            }
            (S(s), S(patt)) => {
                // TODO: not yet defined
                let arr = s.split(patt).map(|s| S(s.to_string())).collect();
                Json::Array(arr)
            }
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(DivFail, msg: "{} / {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Rem for Json {
    type Output = Json;

    fn rem(self, rhs: Json) -> Json {
        use crate::json::Json::{Float, Integer, Null};

        match (&self, &rhs) {
            (Null, _) => Json::Null,
            (_, Null) => Json::Null,
            (Integer(_), Integer(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_integer_result());
                if r == 0 {
                    Null
                } else {
                    Json::new(l % r)
                }
            }
            (Float(_), Float(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_float_result());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new(l % r)
                }
            }
            (Integer(_), Float(_)) => {
                let l = check_error!(self.to_integer_result());
                let r = check_error!(rhs.to_float_result());
                if r == 0_f64 {
                    Null
                } else {
                    Json::new((l as f64) % r)
                }
            }
            (Float(_), Integer(_)) => {
                let l = check_error!(self.to_float_result());
                let r = check_error!(rhs.to_integer_result());
                if r == 0 {
                    Null
                } else {
                    Json::new(l % (r as f64))
                }
            }
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(RemFail, msg: "{} % {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Neg for Json {
    type Output = Json;

    fn neg(self) -> Json {
        match self {
            Json::Null => Json::Null,
            Json::Integer(_) => match self.to_integer_result() {
                Ok(val) => Json::new(-val),
                Err(err) => Json::__Error(err),
            },
            Json::Float(_) => match self.to_float_result() {
                Ok(val) => Json::new(-val),
                Err(err) => Json::__Error(err),
            },
            _ => Json::__Error(
                (err_at!(NegFail, msg: "-{}", self.type_name()) as Result<()>)
                    .unwrap_err(),
            ),
        }
    }
}

impl Shl for Json {
    type Output = Json;

    fn shl(self, rhs: Json) -> Json {
        match (self.to_integer(), rhs.to_integer()) {
            (Some(l), Some(r)) => Json::new(l << r),
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(ShlFail, msg: "{} << {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl Shr for Json {
    type Output = Json;

    fn shr(self, rhs: Json) -> Json {
        match (self.to_integer(), rhs.to_integer()) {
            (Some(l), Some(r)) => Json::new(l >> r),
            (_, _) => {
                let (x, y) = (self.type_name(), rhs.type_name());
                Json::__Error(
                    (err_at!(ShrFail, msg: "{} >> {}", x, y) as Result<()>).unwrap_err(),
                )
            }
        }
    }
}

impl BitAnd for Json {
    type Output = Json;

    fn bitand(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                let x = check_error!(x.to_integer_result());
                let y = check_error!(y.to_integer_result());
                Json::from(x & y)
            }
            (x, y) => {
                let (x, y) = (bool::from(x), bool::from(y));
                Json::from(x & y)
            }
        }
    }
}

impl BitOr for Json {
    type Output = Json;

    fn bitor(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                let x = check_error!(x.to_integer_result());
                let y = check_error!(y.to_integer_result());
                Json::from(x | y)
            }
            (x, y) => {
                let (x, y) = (bool::from(x), bool::from(y));
                Json::from(x | y)
            }
        }
    }
}

impl BitXor for Json {
    type Output = Json;

    fn bitxor(self, rhs: Json) -> Json {
        use crate::json::Json::Integer;

        match (self, rhs) {
            (x @ Json::__Error(_), _) => x,
            (_, y @ Json::__Error(_)) => y,
            (x @ Integer(_), y @ Integer(_)) => {
                let x = check_error!(x.to_integer_result());
                let y = check_error!(y.to_integer_result());
                Json::from(x ^ y)
            }
            (x, y) => {
                let (x, y) = (bool::from(x), bool::from(y));
                Json::from(x ^ y)
            }
        }
    }
}

impl Not for Json {
    type Output = Json;

    fn not(self) -> Json {
        if self.is_error() {
            return self;
        }
        let value = bool::from(self);
        Json::from(value.not())
    }
}

lazy_static! {
    pub static ref INDEX_OUT_OF_BOUND: Json =
        Json::__Error(Error::IndexOutofBound("ops.rs".to_string(), "-1".to_string()));
    pub static ref NOT_AN_ARRAY: Json =
        Json::__Error(Error::NotAnArray("ops.rs".to_string(), "--na--".to_string()));
    pub static ref NOT_AN_INDEX: Json =
        Json::__Error(Error::InvalidIndex("ops.rs".to_string(), "--na--".to_string()));
    pub static ref NOT_A_CONTAINER: Json = Json::__Error(Error::InvalidContainer(
        "ops.rs".to_string(),
        "--na--".to_string()
    ));
    pub static ref PROPERTY_NOT_FOUND: Json = Json::__Error(Error::PropertyNotFound(
        "ops.rs".to_string(),
        "--na--".to_string()
    ));
}

impl Index<isize> for Json {
    type Output = Json;

    fn index(&self, index: isize) -> &Json {
        match self {
            Json::Array(arr) => match normalized_offset(index, arr.len()) {
                Some(off) => &arr[off],
                None => &INDEX_OUT_OF_BOUND,
            },
            Json::__Error(_) => self,
            _ => &NOT_AN_ARRAY,
        }
    }
}

impl Index<&str> for Json {
    type Output = Json;

    fn index(&self, index: &str) -> &Json {
        match self {
            Json::Object(obj) => match obj.binary_search_by(|p| p.as_key().cmp(index)) {
                Ok(off) => obj[off].as_value(),
                Err(_) => &PROPERTY_NOT_FOUND,
            },
            Json::Array(arr) => match index.parse::<isize>() {
                Ok(n) => match normalized_offset(n, arr.len()) {
                    Some(off) => &arr[off],
                    None => &INDEX_OUT_OF_BOUND,
                },
                Err(_) => &NOT_AN_INDEX,
            },
            Json::__Error(_) => self,
            _ => &NOT_A_CONTAINER,
        }
    }
}

pub(crate) fn index_mut<'a>(val: &'a mut Json, key: &str) -> Result<&'a mut Json> {
    match val {
        Json::Object(obj) => match obj.binary_search_by(|p| p.as_key().cmp(key)) {
            Ok(off) => Ok(obj[off].as_mut_value()),
            Err(_) => err_at!(PropertyNotFound, msg: "{}", key.to_string()),
        },
        Json::Array(arr) => match key.parse::<isize>() {
            Ok(n) => match normalized_offset(n, arr.len()) {
                Some(off) => Ok(&mut arr[off]),
                None => err_at!(IndexOutofBound, msg: "{}", n),
            },
            Err(err) => err_at!(InvalidIndex, msg: "{}", err.to_string()),
        },
        Json::__Error(_) => Ok(val),
        _ => err_at!(InvalidContainer, msg: "{}", val.type_name()),
    }
}

fn mixin_object(mut this: Vec<Property>, other: Vec<Property>) -> Vec<Property> {
    use crate::json::Json::Object;

    for o in other.into_iter() {
        match this.binary_search_by(|p| p.as_key().cmp(o.as_key())) {
            Ok(off) => match (this[off].clone().into_value(), o.clone().into_value()) {
                (Object(val), Object(val2)) => {
                    this[off].set_value(Object(mixin_object(val, val2)))
                }
                _ => this.insert(off, o),
            },
            Err(off) => this.insert(off, o.clone()),
        }
    }
    this
}

pub fn normalized_offset(off: isize, len: usize) -> Option<usize> {
    let len = isize::try_from(len).unwrap();
    let off = if off < 0 { off + len } else { off };
    if off >= 0 && off < len {
        Some(usize::try_from(off).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
#[path = "ops_test.rs"]
mod ops_test;
