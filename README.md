Why yet another json package in Rust ?
======================================

[![Rustdoc](https://img.shields.io/badge/rustdoc-hosted-blue.svg)](https://docs.rs/jsondata)
[![GitPitch](https://gitpitch.com/assets/badge.svg)](https://gitpitch.com/bnclabs/jsondata/master?grs=github)
[![Build Status](https://travis-ci.org/bnclabs/jsondata.svg?branch=master)](https://travis-ci.org/bnclabs/jsondata)

This crate makes several trade-offs that are tuned for bigdata
and document database.

* [x] Support for 128-bit signed integers.
* [x] Deferred conversion for JSON numbers.
* [x] Serialization from Rust native type to JSON text.
* [x] De-serialization from JSON text to Rust native type.
* [x] CRUD operation on JSON documents, using [JSON Pointer][jptr].
* [x] Sorted keys in property object.
* [ ] Streaming JSON parser.
* [x] Support [JSON5](http://json5.org) standard.
* [ ] Common arithmetic and logic operations.
* [ ] Sortable JSON.

**[API Documentation](https://docs.rs/jsondata)**

Deferred conversion for numbers
===============================

Converting JSON numbers to Rust native type is not always desired.
Especially in the context of bigdata where data is stored in JSON
format and we need to lookup, only, specific fields within the document.

This implementation provides deferred conversion for JSON numbers
that leads to a **[performance improvement of upto 30%][commit-deferred]**.

CRUD operations on JSON documents
=================================

Using Json Pointer it is possible to identify a specific field nested within
a JSON document. For Example, with below document:

```json
  {
    "age": 26,
    "eyeColor": "green",
    "name": "Leon Robertson",
    "gender": "male",
    "company": "AEORA",
    "phone": "+1 (835) 447-2960",
    "tags": [ "officia", "reprehenderit", "magna" ],
    "friends": [
      {
        "id": 0,
        "name": "Glenda Chan"
      }
    ]
  }
```

* **/age** shall point to value ``26``.
* **/friends** shall point to value ``[{"id": 0, "name": "Glenda Chan"}]``,

**List of operations**

* [x] Get a field nested within a JSON document using [JSON Pointer][jptr].
* [x] Set a field nested within a JSON document.
* [x] Delete a field nested within a JSON document.
* [x] Append string or array field withing a JSON document.

JSON5
=====

* [x] Object keys may be an ECMAScript 5.1 IdentifierName.
* [x] Objects may have a single trailing comma.
* [x] Arrays may have a single trailing comma.
* [ ] Strings may be single quoted.
* [ ] Strings may span multiple lines by escaping new line characters.
* [ ] Strings may include character escapes.
* [x] Numbers may be hexadecimal.
* [x] Numbers may have a leading or trailing decimal point.
* [x] Numbers may be IEEE 754 positive infinity, negative infinity, and NaN.
* [x] Numbers may begin with an explicit plus sign.
* [ ] Single and multi-line comments are allowed.
* [x] Additional white space characters are allowed.

**[Track this activity](https://github.com/bnclabs/jsondata/issues/4)**

Help wanted
===========

* Add readme badges [#1][#1].
* Alternate string parsing for non-unicode JSON [#3][#3].
* JSON5 implementation [#4][#4].

[commit-deferred]: https://github.com/bnclabs/jsondata/commit/70e6dedf0121f16e130f224daaa23948f5a5d782
[jptr]: https://tools.ietf.org/html/rfc6901
[#1]: https://github.com/bnclabs/jsondata/issues/1
[#3]: https://github.com/bnclabs/jsondata/issues/3
[#4]: https://github.com/bnclabs/jsondata/issues/4
