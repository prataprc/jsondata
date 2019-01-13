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

0.3.0
=====

* JSON5 support. Most of JSON5 specification, especially those
that are relavant for bigdata document database, are implemented.
* Added release-checklist
* Bugfixes
* Implement AsRef and AsMut traits for Json type.
* Travis-CI integration for ``clippy``.

Release Checklist
=================

* Bump up the version:
  * __major__: backward incompatible API changes.
  * __minor__: backward compatible API Changes.
  * __patch__: bug fixes.
* Travis-CI integration.
* Cargo checklist
  * cargo +stable build; cargo +nightly build
  * cargo +nightly clippy --all-targets --all-features
  * cargo doc
  * cargo +nightly test
  * cargo +nightly bench
  * cargo +nightly benchcmp <old> <new>
  * cargo fix --edition --all-targets
* Create a git-tag for the new version.
* Cargo publish the new version.
* Badges
  * rust-doc
  * gitpitch
  * build-passing
