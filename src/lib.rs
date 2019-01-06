//! Jsondata is yet another [JSON] implementation in Rust, but tuned
//! and optimized for bigdata and document-databases that store
//! documents in [JSON] format. Following is the scope defined
//! for this package:
//!
//! * Support for 128-bit signed integers.
//! * Deferred conversion of numbers.
//! * Serialization from Rust native type to JSON text.
//! * De-serialization from JSON text to Rust native type.
//! * [CRUD] operation on JSON documents, using [JSON Pointer].
//! * Sorted keys in property object.
//! * Streaming JSON parser.
//! * Support JSON5 standard.
//! * Common arithmetic and logic operations.
//! * Sortable JSON.
//!
//! To parse JSON text, use [parse]:
//!
//! ```
//! extern crate jsondata;
//! use jsondata::Json;
//!
//! let text = r#"[null,true,false,10,"true"]"#;
//! let json = text.parse::<Json>().unwrap();
//! ```
//!
//! To serialise [Json] type to JSON text:
//!
//! ```
//! extern crate jsondata;
//! use jsondata::Json;
//!
//! let text = r#"[null,true,false,10,"true"]"#;
//! let json = text.parse::<Json>().unwrap();
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
//! extern crate jsondata;
//! use jsondata::Json;
//!
//! let mut json = "1000".parse::<Json>().unwrap();
//! json.integer().unwrap(); // "1000" is parsed
//! json.integer().unwrap(); // "1000" is parsed again
//!
//! match json.compute() { // pre-compute all numbers in the json document.
//!     Ok(_) => (),
//!     Err(s) => println!("{}", s),
//! }
//! ```
//!
//! If JSON text is going to come from un-trusted parties,
//!
//! ```
//! extern crate jsondata;
//! use jsondata::Json;
//!
//! let mut json = r#"{"a": 1000}"#.parse::<Json>().unwrap();
//! match json.validate() { // validate
//!     Ok(_) => (),
//!     Err(s) => println!("{}", s),
//! }
//! ```
//!
//! [JSON]: https://tools.ietf.org/html/rfc8259
//! [CRUD]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
//! [JSON Pointer]: https://tools.ietf.org/html/rfc6901
//! [parse]: str::method.parse
//! [integer]: enum.Json.html#method.integer
//! [float]: enum.Json.html#method.float

#![feature(test)]
#![feature(plugin)]
#![feature(vec_remove_item)]
#![feature(exclusive_range_pattern)]
#![plugin(quickcheck_macros)]

#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate quickcheck;
extern crate test;
extern crate unicode_reader;

mod json;
mod lex;
mod num;
mod ops;
mod parse;
mod property; // TODO: should we rename this as "property"

pub mod jptr;

// Re-exports for API documentation.
pub use json::{Json, Jsons};
pub use property::Property;

// TODO: Remove this once quickcheck is fully added for testing.
//#[cfg(test)]
//mod tests {
//    #[quickcheck]
//    fn double_reversal_is_identity(_xs: i8) -> bool {
//        true
//    }
//}

#[cfg(test)]
mod jptr_test;
#[cfg(test)]
mod json_test;
