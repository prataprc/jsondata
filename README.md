Why yet another json package in Rust ?
======================================

[![Rustdoc](https://img.shields.io/badge/rustdoc-hosted-blue.svg)](https://docs.rs/jsondata)
[![GitPitch](https://gitpitch.com/assets/badge.svg)](https://gitpitch.com/bnclabs/jsondata/master?grs=github)

This crate makes several trade-offs that are tuned for bigdata
and document database.

* [x] Support for 128-bit signed integers.
* [x] Deferred conversion of integer / float.
* [x] Serialization from Rust native type to JSON text.
* [x] De-serialization from JSON text to Rust native type.
* [ ] Sorted keys in property object.
* [ ] Streaming JSON parser.
* [ ] Support [JSON5](json5.org) standard.
* [ ] Common arithmetic and logic operations.
* [ ] Sortable JSON.

Deferred conversion for numbers
===============================

Converting JSON numbers to Rust native type is not always desired.
Especially in the context of bigdata where data is stored in JSON
format and we need to lookup, only, specific fields within the document.

This implementation provides deferred conversion for JSON numbers
that leads to a [performance improvement of upto 30%](commit-deferred).

[commit-deferred]: https://github.com/bnclabs/jsondata/commit/70e6dedf0121f16e130f224daaa23948f5a5d782
