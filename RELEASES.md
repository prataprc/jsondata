0.1.0
=====

* Support for 128-bit signed integers.
* Deferred conversion for JSON numbers.
* Serialization from Rust native type to JSON text.
* De-serialization from JSON text to Rust native type.
* Sorted keys in property object.

0.2.0
=====

* CRUD operation on JSON documents, using JSON Pointer.

Release Checklist
=================

* Bump up the version:
  * __major__: backward incompatible API changes.
  * __minor__: backward compatible API Changes.
  * __patch__: bug fixes.
* Travis-CI integration.
* Cargo checklist
  * cargo test
  * cargo bench
  * cargo doc
  * cargo benchcmp between older version and new version.
  * cargo clippy
* Create a git-tag for the new version.
* Cargo publish the new version.
* Badges
  * rust-hosted
  * gitpitch
  * build-passing
