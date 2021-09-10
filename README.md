Why yet another JSON package in Rust ?
======================================

[![Rustdoc](https://img.shields.io/badge/rustdoc-hosted-blue.svg)](https://docs.rs/jsondata)
[![GitPitch](https://gitpitch.com/assets/badge.svg)](https://gitpitch.com/bnclabs/jsondata/master?grs=github)
[![Build Status](https://travis-ci.org/bnclabs/jsondata.svg?branch=master)](https://travis-ci.org/bnclabs/jsondata)

This crate makes several trade-offs that are tuned for big-data
and document database.

* [x] Support for 128-bit signed integers.
* [x] Deferred conversion for JSON numbers.
* [x] Serialization from Rust native type to JSON text.
* [x] De-serialization from JSON text to Rust native type.
* [x] CRUD operation on JSON documents, using [JSON Pointer][jptr].
* [x] Sorted keys in property object.
* [x] Streaming JSON parser.
* [x] Support [JSON5](http://json5.org) standard.
* [x] Common arithmetic and logic operations.
* [x] Sortable JSON.

**Useful links**

* **[API Documentation](https://docs.rs/jsondata)**
* [JSON Pointer][jptr].
* [JSON5][json5].
* Rust [internal discussion][rust1]  on f64 -> integer.
* [Json sort order][json-sort-order].
* [Json operations][json-ops].
* [Release notes](./RELEASE.md).

Deferred conversion for numbers
===============================

Converting JSON numbers to Rust native type is not always desired.
Especially in the context of big-data where data is stored in JSON
format and we need to lookup, only, specific fields within the document.

This implementation provides deferred conversion for JSON numbers
that leads to a **[performance improvement of upto 30%][commit-deferred]**.

CRUD operations on JSON document
================================

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
* **/tags** shall point to value ``[ "officia", "reprehenderit", "magna" ]``.
* **/tags/0** shall point to value ``"officia"``.
* **/friends** shall point to value ``[{"id": 0, "name": "Glenda Chan"}]``.
* **/friends/name** shall point to value ``"Glenda Chan"``.

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

**[Track this feature](https://github.com/bnclabs/jsondata/issues/4)**.

Sortable JSON
=============

* **Null** type shall sort before all other types.
* **Boolean** type shall sort after Null type.
* **Number** type shall sort after Boolean type.
  * f64 values that are <= -2^127 will sort before all i128 integers.
  * f64 values that are >= 2^127-1 will sort after all i128 integers.
  * NaN, Not a Number, values shall sort after all i128 integers
  * **-Infinity** shall sort before all numbers.
  * **+Infinity** shall sort after all numbers.
  * **NaN** shall sort after +Infinity.
* **String** type shall sort after Number type.
* **Array** type shall sort after String type.
* **Object** type shall sort after Array type.
  * All (key,value) pairs within the object shall be presorted based
    on the key.
  * When comparing two objects, comparison shall start from first key
    and proceed to the last key.
  * If two keys are equal at a given position within the objects, then
    its corresponding values shall be compared.
  * When one object is a subset of another object, as in, if one object
    contain all the (key,value) properties that the other object has
    then it shall sort before the other object.

**Useful links**

- **[A detailed description of JSON sort order][json-sort-order]**.
- Rust-lang [issue#46298](https://github.com/rust-lang/rust/issues/46298) and
  [issue#10184](https://github.com/rust-lang/rust/issues/10184),
  discussing saturating cast of f64 -> integer.
- Rust [internal discussion][rust1]  on f64 -> integer.
- Unicode collation [TR10](http://unicode.org/reports/tr10).
- [ICU collation](http://userguide.icu-project.org/collation).
- Floating point [Total ordering][fp-total-order]
- Total ordering for floating point in [stackoverflow][sf1].
- [Total ordering thread](https://users.rust-lang.org/t/sort-order-for-json/24166)
  in http://users.rust-lang.org.
- A good [blog][blog1] on floating point, to get started.

Operations on JSON documents
============================

* Arithmetic operations, ADD, SUB, MUL, DIV, REM, NEG.
* Bitwise operations, SHL, SHR, BITAND, BITOR, BITXOR.
* Logical operations, NOT, AND, OR.
* Index operations.
* Range operations.

**[Detailed description can be found here][json-ops].**

Contribution
------------

* Simple workflow. Fork - Modify - Pull request.
* Before creating a PR,
  * Run `make build` to confirm all versions of build is passing with
    0 warnings and 0 errors.
  * Run `check.sh` with 0 warnings, 0 errors and all testcases passing.
  * Run `perf.sh` with 0 warnings, 0 errors and all testcases passing.
  * [Install][spellcheck] and run `cargo spellcheck` to remove common spelling mistakes.
* [Developer certificate of origin][dco] is preferred.

[commit-deferred]: https://github.com/bnclabs/jsondata/commit/70e6dedf0121f16e130f224daaa23948f5a5d782
[json5]: http://json5.org
[jptr]: https://tools.ietf.org/html/rfc6901
[#1]: https://github.com/bnclabs/jsondata/issues/1
[#3]: https://github.com/bnclabs/jsondata/issues/3
[#4]: https://github.com/bnclabs/jsondata/issues/4
[#13]: https://github.com/bnclabs/jsondata/issues/13
[json-sort-order]: https://prataprc.github.io/json-sort-order.html
[json-ops]: https://prataprc.github.io/json-operations.html
[rust1]: https://internals.rust-lang.org/t/help-us-benchmark-saturating-float-casts/6231
[sf1]: https://stackoverflow.com/questions/8341395/what-is-a-subnormal-floating-point-number
[fp-total-order]: https://en.m.wikipedia.org/wiki/IEEE_754#Total-ordering_predicate
[blog1]: https://steve.hollasch.net/cgindex/coding/ieeefloat.html
[MIT license]: https://opensource.org/licenses/MIT
[spellcheck]: https://github.com/drahnr/cargo-spellcheck
[dco]: https://developercertificate.org/
