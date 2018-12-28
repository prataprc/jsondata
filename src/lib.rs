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
//! [JSON]: https://tools.ietf.org/html/rfc8259
//! [CRUD]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
//! [JSON Pointer]: https://tools.ietf.org/html/rfc6901

#![feature(test)]
#![feature(plugin)]
#![feature(vec_remove_item)]
#![feature(exclusive_range_pattern)]

#![plugin(quickcheck_macros)]

#[cfg(test)] extern crate quickcheck;
extern crate test;

mod lex;
mod property; // TODO: should we rename this as "property"
mod num;
mod json;
mod parse;
mod ops;

pub mod jptr;

// Re-exports for API documentation.
pub use property::Property;
pub use json::Json;

#[cfg(test)]
mod tests {
    #[quickcheck]
    fn double_reversal_is_identity(_xs: i8) -> bool {
        true
    }
}

#[cfg(test)] mod json_test;
#[cfg(test)] mod jptr_test;
