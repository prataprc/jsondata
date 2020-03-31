// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use std::{fmt, result};

/// Enumeration of all possible errors that shall be returned by
/// methods and functions under this package. Refer to individual
/// methods and functions, returning [Result] type, for specific
/// error handling.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Failed to parse JSON text.
    ParseFail(String),
    /// Failed to add two Json values.
    AddFail(String),
    /// Failed to subract two Json values.
    SubFail(String),
    /// Failed to multiply two Json values.
    MulFail(String),
    /// Failed to divide two Json values.
    DivFail(String),
    /// Failed to find reminder between two Json values.
    RemFail(String),
    /// Failed to negate Json value.
    NegFail(String),
    /// Failed to left shift Json value.
    ShlFail(String),
    /// Failed to right shift Json value.
    ShrFail(String),
    /// When indexing a Json array with out of bound index.
    IndexOutofBound(isize),
    /// When attempting index an array with an invalid index.
    InvalidIndex(String),
    /// When attempting array operation, like range, index etc..,
    /// on non-array value.
    NotAnArray(String),
    /// When attempting lookup and indexing operations like, set, delete,
    /// append, index, etc.. on values that are neither array nor object.
    InvalidContainer(String),
    /// Not an expected type, arugment gives the found type.
    InvalidType(String),
    /// When trying to lookup a Json object with missing property.
    PropertyNotFound(String),
    /// While appending a non string value with Json string.
    AppendString(String),
    /// Found JSON text that looks like number, but not well formed.
    InvalidNumber(String),
    /// Failed processing json-pointer.
    JptrFail(String),
    /// std::io::Error returned by string processing API, while iterating
    /// on [`crate::Jsons`] stream of text.
    IoError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::ParseFail(m) => write!(f, "ParseFail:{}", m),
            Error::AddFail(m) => write!(f, "AddFail:{}", m),
            Error::SubFail(m) => write!(f, "SubFail:{}", m),
            Error::MulFail(m) => write!(f, "MulFail:{}", m),
            Error::DivFail(m) => write!(f, "DivFail:{}", m),
            Error::RemFail(m) => write!(f, "RemFail:{}", m),
            Error::NegFail(m) => write!(f, "NegFail:{}", m),
            Error::ShlFail(m) => write!(f, "ShlFail:{}", m),
            Error::ShrFail(m) => write!(f, "ShrFail:{}", m),
            Error::IndexOutofBound(m) => write!(f, "IndexOutofBound:{}", m),
            Error::InvalidIndex(m) => write!(f, "InvalidIndex:{}", m),
            Error::NotAnArray(m) => write!(f, "NotAnArray:{}", m),
            Error::InvalidContainer(m) => write!(f, "InvalidContainer:{}", m),
            Error::InvalidType(m) => write!(f, "InvalidType:{}", m),
            Error::PropertyNotFound(m) => write!(f, "PropertyNotFound:{}", m),
            Error::AppendString(m) => write!(f, "AppendString:{}", m),
            Error::InvalidNumber(m) => write!(f, "InvalidNumber:{}", m),
            Error::JptrFail(m) => write!(f, "JptrFail:{}", m),
            Error::IoError(m) => write!(f, "IoError:{}", m),
        }
    }
}

/// Result type, for jsondata functions and methods, that require a
/// success or failure variant.
pub type Result<T> = std::result::Result<T, Error>;
