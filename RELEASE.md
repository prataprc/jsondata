0.8.0
=====

* **Breaking change**: Json method rename `typename()` method to `type_name()`.
* implement `err_at!()` macro and Debug for Error type.
* parse: Json parser, test fix and refactoring.
  * optimize `parse_identifier()` implementation.
  * fix testcase for parsing float number.
* refactor `num.rs`.
* replace `search_by_key()` with `[T].binary_search_by()` for `Property` array.
  `stdlib` has an equivalent implementation to find a key in a sorted
  array. Use that instead of `search_by_key()`.
* ops.rs: where possible capture error instead of panic.
* reimplement traits for Json type using `macro_rules`.
* rustdoc.
* update CI scripts.

0.7.0
=====

* move license back to MIT
* Json type: export typename() API.
* implement conversion traits from primitive types to Json.
* Error type: implement Display trait.
* Error type: add InvalidType enum-variant.
* JsonSerialize: procedural macro.
* idiomatic rust API.
* clippy fixes.

0.6.2
=====

Breaking type change.
- Integral and Floating types are implemented as enum, instead of struct.

- Fix "as ..." conversions to try_into().unwrap()
- Implement From<usize>, From<u64>, From<u32> for Json type.

0.6.1
=====

- rustdoc
- clippy fixes

0.6.0
=====

Breaking API Change:

Previously APIs returning Result had its Err variant as String type.
Now we are implementing a proper Error type and returning that as the
Err variant.

0.5.0
=====

- Documentation.
- Move license to AGPL-3.0

0.4.0
=====

* Streaming JSON parser.
* Fixes to travis, stable and nightly rust channels.
* Implement PartialOrd for Json type.
* Total ordering for Json type.
* Implement Arithmetic/logical/bitwise traits for Json type.
* Implement Range operation for Json type.

0.3.0
=====

* JSON5 support. Most of JSON5 specification, especially those
that are relevant for big-data document database, are implemented.
* Added release-checklist
* Bug fixes
* Implement AsRef and AsMut traits for Json type.
* Travis-CI integration for ``clippy``.

0.2.0
=====

* CRUD operation on JSON documents, using JSON Pointer.

0.1.0
=====

* Support for 128-bit signed integers.
* Deferred conversion for JSON numbers.
* Serialization from Rust native type to JSON text.
* De-serialization from JSON text to Rust native type.
* Sorted keys in property object.

Code Review checklist
=====================

* [ ] Review and check for un-necessary copy, and allocations.
* [ ] Review resize calls on `Vec`.
* [ ] Review (as ...) type casting, to panic on data loss.
* [ ] Reduce trait constraints for Type parameters on public APIs.
* [ ] Public APIs can be as generic as possible. Check whether there
      is a scope for `AsRef` or `Borrow` constraints.
* [ ] Document error variants.
* [ ] Check for dangling links in rustdoc.

Refer to [release-checklist][release-checklist].

[release-checklist]: https://prataprc.github.io/rust-crates-release-checklist.html
