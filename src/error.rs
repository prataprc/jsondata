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
    /// Failed to index into Json value.
    IndexFail(String),
    /// When indexing a Json array with out of bound index.
    IndexOutofBound(isize),
    /// When parsing an invalid array index.
    NotAnIndex(String),
    /// When Json value is expected to be an array, but it is not.
    NotAnArray(String),
    /// When Json value is expected to be an object, but it is not.
    NotAnObject(String),
    /// When json-pointer is stuck with non-container type. Only array
    /// and object are treated as container type.
    NotAContainer(String),
    /// When trying to lookup a Json object with missing property.
    PropertyNotFound(String),
    /// While appending a non string value with Json string.
    AppendString(String),
    /// Found JSON text that looks like number, but not well formed.
    InvalidNumber(String),
    /// Failed processing json-pointer.
    JptrFail(String),
    /// std::io::Error
    IoError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
