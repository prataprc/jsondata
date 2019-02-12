Tip
===

[On going development]

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
that are relavant for bigdata document database, are implemented.
* Added release-checklist
* Bugfixes
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

Release Checklist
=================

* Bump up the version:
  * __major__: backward incompatible API changes.
  * __minor__: backward compatible API Changes.
  * __patch__: bug fixes.
* Cargo checklist
  * cargo +stable build; cargo +nightly build
  * cargo +stable doc
  * cargo +nightly clippy --all-targets --all-features
  * cargo +nightly test
  * cargo +nightly bench
  * cargo +nightly benchcmp <old> <new>
  * cargo fix --edition --all-targets
* Travis-CI integration.
* Create a git-tag for the new version.
* Cargo publish the new version.
* Badges
  * Build passing, Travis continuous integration.
  * Code coverage, codecov and coveralls.
  * Crates badge
  * Downloads badge
  * License badge
  * Rust version badge.
  * Maintenance-related badges based on isitmaintained.com
  * Documentation
  * Gitpitch
