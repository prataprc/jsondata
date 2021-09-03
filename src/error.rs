// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use std::{error, fmt, result};

/// Short form to compose Error values.
///
/// Here are few possible ways:
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, msg: format!("bad argument"));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, std::io::read(buf));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, std::fs::read(file_path), format!("read failed"));
/// ```
///
#[macro_export]
macro_rules! err_at {
    ($v:ident, msg: $($arg:expr),+) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!($($arg),+)))
    }};
    ($v:ident, $e:expr) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    }};
    ($v:ident, $e:expr, $($arg:expr),+) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                let msg = format!($($arg),+);
                Err(Error::$v(prefix, format!("{} {}", err, msg)))
            }
        }
    }};
}

/// Enumeration of all possible errors that shall be returned by this package.
///
/// Refer to individual methods and functions, returning [Result] type, for
/// specific error handling.
#[derive(Clone, PartialEq)]
pub enum Error {
    /// Failed to parse JSON text.
    ParseFail(String, String),
    /// Failed to add two Json values.
    AddFail(String, String),
    /// Failed to subract two Json values.
    SubFail(String, String),
    /// Failed to multiply two Json values.
    MulFail(String, String),
    /// Failed to divide two Json values.
    DivFail(String, String),
    /// Failed to find reminder between two Json values.
    RemFail(String, String),
    /// Failed to negate Json value.
    NegFail(String, String),
    /// Failed to left shift Json value.
    ShlFail(String, String),
    /// Failed to right shift Json value.
    ShrFail(String, String),
    /// When indexing a Json array with out of bound index.
    IndexOutofBound(String, String),
    /// When attempting index an array with an invalid index.
    InvalidIndex(String, String),
    /// When attempting array operation, like range, index etc..,
    /// on non-array value.
    NotAnArray(String, String),
    /// When attempting lookup and indexing operations like, set, delete,
    /// append, index, etc.. on values that are neither array nor object.
    InvalidContainer(String, String),
    /// Not an expected type, arugment gives the found type.
    InvalidType(String, String),
    /// When trying to lookup a Json object with missing property.
    PropertyNotFound(String, String),
    /// While appending a non string value with Json string.
    AppendString(String, String),
    /// Found JSON text that looks like number, but not well formed.
    InvalidNumber(String, String),
    /// Failed processing json-pointer.
    JptrFail(String, String),
    /// std::io::Error returned by string processing API, while iterating
    /// on [`crate::Jsons`] stream of text.
    IoError(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::ParseFail(p, m) => write!(f, "{} ParseFail:{}", p, m),
            Error::AddFail(p, m) => write!(f, "{} AddFail:{}", p, m),
            Error::SubFail(p, m) => write!(f, "{} SubFail:{}", p, m),
            Error::MulFail(p, m) => write!(f, "{} MulFail:{}", p, m),
            Error::DivFail(p, m) => write!(f, "{} DivFail:{}", p, m),
            Error::RemFail(p, m) => write!(f, "{} RemFail:{}", p, m),
            Error::NegFail(p, m) => write!(f, "{} NegFail:{}", p, m),
            Error::ShlFail(p, m) => write!(f, "{} ShlFail:{}", p, m),
            Error::ShrFail(p, m) => write!(f, "{} ShrFail:{}", p, m),
            Error::IndexOutofBound(p, m) => write!(f, "{} IndexOutofBound:{}", p, m),
            Error::InvalidIndex(p, m) => write!(f, "{} InvalidIndex:{}", p, m),
            Error::NotAnArray(p, m) => write!(f, "{} NotAnArray:{}", p, m),
            Error::InvalidContainer(p, m) => write!(f, "{} InvalidContainer:{}", p, m),
            Error::InvalidType(p, m) => write!(f, "{} InvalidType:{}", p, m),
            Error::PropertyNotFound(p, m) => write!(f, "{} PropertyNotFound:{}", p, m),
            Error::AppendString(p, m) => write!(f, "{} AppendString:{}", p, m),
            Error::InvalidNumber(p, m) => write!(f, "{} InvalidNumber:{}", p, m),
            Error::JptrFail(p, m) => write!(f, "{} JptrFail:{}", p, m),
            Error::IoError(p, m) => write!(f, "{} IoError:{}", p, m),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

/// Result type, for jsondata functions and methods, that require a
/// success or failure variant.
pub type Result<T> = std::result::Result<T, Error>;
