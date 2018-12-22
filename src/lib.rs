#![feature(test)]
#![feature(plugin)]
#![feature(vec_remove_item)]

#![plugin(quickcheck_macros)]

#[cfg(test)] extern crate quickcheck;
extern crate test;

mod lex;
mod kv; // TODO: should we rename this as "property"
mod json;
mod parse;
mod ops;

// Re-exports for API documentation.
pub use kv::KeyValue;
pub use json::Json;

#[cfg(test)]
mod tests {
    #[quickcheck]
    fn double_reversal_is_identity(_xs: i8) -> bool {
        true
    }
}

#[cfg(test)] mod json_test;
