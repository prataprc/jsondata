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

/// Result type, for jsondata functions and methods, that require a
/// success or failure variant.
pub type Result<T> = std::result::Result<T, Error>;
