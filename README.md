Why yet another json package for Rust ?

This crate makes several tradeoffs that are tuned for bigdata
and document database.

* [x] Support for 128-bit signed integers.
* [x] Deferred conversion of integer / float.
* [x] Serialisation from Rust native type to JSON text.
* [x] Deserialisation from JSON text to Rust native type.
* [ ] Sorted keys in property object.
* [ ] Streaming JSON parser.
* [ ] Support [JSON5](json5.org) standard.
* [ ] Common arithmetic and logic operations.
* [ ] Sortable JSON.

**Deferred conversion for integer / float**

Converting JSON numbers to Rust native type is not always desired.
Especially in the context of bigdata where data is stored in JSON
format and we need to lookup specific fields within the document.

This implementation provides deferred conversion for JSON numbers
that leads to a performance improvement of upto 30%.
