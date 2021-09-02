// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

//! Jsondata is yet another [JSON] implementation in Rust, but
//! optimized for big-data and document-databases that store
//! documents in [JSON] format. Following is the scope defined
//! for this package:
//!
//! * Support for 128-bit signed integers.
//! * Deferred conversion of numbers.
//! * Serialization from Rust native type, [`Json`], to JSON text.
//! * De-serialization from JSON text to Rust native [`Json`] type.
//! * [CRUD] operation on JSON documents, using [JSON Pointer].
//! * Sorted keys in property object.
//! * Streaming JSON parser, using [`Jsons`] type.
//! * Support [JSON5](http://json5.org) standard.
//! * Common arithmetic and logical ops implemented for [`Json`].
//! * [`Json`] objects can be compared and sorted.
//!
//! To parse JSON text, use [`str::parse`]:
//!
//! ```
//! let text = r#"[null,true,false,10,"true"]"#;
//! let json = text.parse::<jsondata::Json>().unwrap();
//! ```
//!
//! To serialise [`Json`] type to JSON text:
//!
//! ```
//! let text = r#"[null,true,false,10,"true"]"#;
//! let json = text.parse::<jsondata::Json>().unwrap();
//!
//! let text1 = json.to_string();    // one way to serialize to JSON
//! let text2 = format!("{}", json); // another way to serialize to JSON
//! assert_eq!(text1, text2);
//! ```
//!
//! When parsing a JSON text to [Json] instance, numbers are not parsed
//! right away, hence calls to [integer] and [float] methods will have
//! to compute the value every time,
//!
//! ```
//! let mut json = "1000".parse::<jsondata::Json>().unwrap();
//! json.to_integer().unwrap(); // "1000" is parsed
//! json.to_integer().unwrap(); // "1000" is parsed again
//!
//! match json.compute() { // pre-compute all numbers in the json document.
//!     Ok(_) => (),
//!     Err(s) => println!("{:?}", s),
//! }
//! ```
//!
//! If JSON text is going to come from untrusted parties,
//!
//! ```
//! let mut json = r#"{"a": 1000}"#.parse::<jsondata::Json>().unwrap();
//! match json.validate() { // validate
//!     Ok(_) => (),
//!     Err(s) => println!("{:?}", s),
//! }
//! ```
//!
//! Boolean table for [`Json`] data:
//! ================================
//!
//! In boolean expression context, like BitAnd, BitOr, BitXor and Not,
//! [`Json`] value will be automatically converted to boolean value
//! with following rules.
//!
//! |      data type    | boolean value |
//! |-------------------|---------------|
//! | null              | false         |
//! | boolean true      | true          |
//! | boolean false     | false         |
//! | integer-zero      | false         |
//! | integer-non-zero  | true          |
//! | float-zero        | false         |
//! | float-non-zero    | true          |
//! | string-empty      | false         |
//! | string            | true          |
//! | array-empty       | false         |
//! | array-non-empty   | true          |
//! | object-empty      | false         |
//! | object-non-empty  | true          |
//!
//!
//! JSON operations:
//! ================
//!
//! [`Json`] implements common arithmetic and logical operations like
//! [Add], [Sub], [Mul], [Div], [Rem], [Neg], [Shl], [Shr], [BitAnd],
//! [BitOr], [BitXor], [Not], [Index].
//!
//! *Addition:*
//!
//! * Adding with Null, shall return the same value.
//! * Integer addition and Float addition respectively follow
//!   i128 and f64 rules. When adding mixed numbers, integers
//!   are type casted to floats.
//! * Adding two string, shall concatenate and return a new string.
//! * Adding two array, shall return a new array with first array's
//!   element and later second array's element.
//! * Adding two object, is similar to adding two array except that if
//!   both object have same property, then property from first object
//!   is overwritten by property from second object.
//!
//! All other combination shall return [`Error::AddFail`].
//!
//! *Subraction:*
//!
//! * Subracting with Null, shall return the same value.
//! * Integer subraction and Float subraction respectively follow
//!   i128 and f64 rules. When subracting mixed numbers, integers
//!   are type casted to floats.
//! * Subracting an array from another array, shall return a new array
//!   with remaining items after removing second array's item from the
//!   the first array.
//! * Subracting two object, is similar to subracting two array.
//!
//! All other combination shall return [`Error::SubFail`].
//!
//! *Multiplication:*
//!
//! * Multiplying with Null, shall return Null.
//! * Integer multiplication and Float multiplication respectively
//!   follow i128 and f64 rules. When multiplying mixed numbers,
//!   integers are type casted to floats.
//! * Multiplying integer with string or vice-versa, shall repeat
//!   the string operand as many times specified by the integer value
//!   and return a new string.
//!
//! All other combination shall return [`Error::MulFail`].
//!
//! *Division:*
//!
//! * Dividing with Null, shall return Null.
//! * Integer division and Float division respectively follow
//!   i128 and f64 rules. When dividing with mixed numbers,
//!   integers are type casted to floats.
//!
//! All other combination shall return [`Error::DivFail`].
//!
//! *Reminder:*
//!
//! * Finding reminder with Null, shall return Null.
//! * Integer reminder and Float reminder respectively follow
//!   i128 and f64 rules. When dividing with mixed numbers,
//!   integers are type casted to floats.
//!
//! All other combination shall return [`Error::RemFail`].
//!
//! *Negation:*
//!
//! * Negating Null, shall return Null.
//! * Negating Integer and Float shall respectively follow
//!   i128 and f64 rules.
//!
//! All other combination shall return [`Error::NegFail`].
//!
//! *Shift-right / Shift-left:*
//!
//! Applicable only for integers and follow the same behaviour as
//! that of [`i128`].
//!
//! All other combination shall return [`Error::ShrFail`] /
//! [`Error::ShlFail`].
//!
//! *BitAnd:*
//!
//! * For integer operands, BitAnd follows normal integer bitwise-and
//!   rules.
//! * For other Json variant, operand is converted to boolean
//!   version and performs logical AND.
//!
//! *BitOr:*
//!
//! * For integer operands, BitOr follows normal integer bitwise-or
//!   rules.
//! * For other Json variant, operand is converted to boolean
//!   version and performs logical OR.
//!
//! *BitXor:*
//!
//! * For integer operands, BitXor follows normal integer bitwise-xor
//!   rules.
//! * For other Json variant, operand is converted to boolean
//!   version and performs logical XOR.
//!
//! [JSON]: https://tools.ietf.org/html/rfc8259
//! [CRUD]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
//! [JSON Pointer]: https://tools.ietf.org/html/rfc6901
//! [integer]: enum.Json.html#method.integer
//! [float]: enum.Json.html#method.float

#![feature(total_cmp)]
#![doc(
    html_favicon_url = "https://cdn4.iconfinder.com/data/icons/fugue/icon_shadowless/json.png"
)]
#![doc(
    html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c9/JSON_vector_logo.svg/1024px-JSON_vector_logo.svg.png"
)]

#[allow(unused_imports)]
use std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, Index, Mul, Neg, Not, Rem, Shl, Shr, Sub,
};

#[doc(hidden)]
pub use jsondata_derive::*;

mod error;
mod json;
mod jsons;
mod lex;
mod num;
mod ops;
mod parse;
mod property;

pub mod jptr;

// Re-exports for API documentation.
pub use crate::error::{Error, Result};
pub use crate::json::Json;
pub use crate::jsons::Jsons;
pub use crate::property::Property;
