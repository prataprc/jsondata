@title[JSON]

@snap[midpoint slide1]
<h1>JSON</h1>
<br/>
<b style="color: crimson;">J</b>ava<b style="color: crimson;">S</b>cript
<b style="color: crimson;">O</b>bject
<b style="color: crimson;">N</b>otation
@snapend


@snap[south-east author-box]
@fa[envelope](prataprc@gmail.com - R Pratap Chakravarthy) <br/>
@fa[github](https://github.com/bnclabs/jsondata) <br/>
@snapend

---

Why JSON ?
==========

<br/>
@snap[fragment text-center text-blue]
Web standard for data exchange.
@snapend

@snap[mt30 fragment text-center text-green]
Inter-operability with several languages.
@snapend

@snap[mt30 fragment text-center text-black]
Human friendly as apposed to machine friendly.
@snapend

---

Data types
==========

JSON defines the least common subset of data types across
sevaral languages.

@snap[fragment mt30]
<b>Primitive types</b>
@snapend

@ul
* **null**, equivalent of None, nil, null in many languages.
* **number**, base 10 representation.
* **bool**, true or false. Both represented in lowercase.
* **string**, double quoted, utf8 encoded plain text.
@ulend

@snap[fragment mt30]
<b>Structured types</b>
@snapend

@ul
* **Array**, heterogenous collection of other types, with number index.
* **Object**, heterogenous collection of other types, with string index.
@ulend


+++

Examples
========

@snap[midpoint]

@snap[fragment]
"Hello world!"
@snapend

@snap[fragment]
42
@snapend

@snap[fragment]
true
@snapend

@snap[fragment]
null
@snapend

+++

Examples: Numbers
=================

@snap[west ml5]
<pre style="line-height: 2em">
132837
-132837
0
.123
0.123
-0.123
-.123
1e2
</pre>
@snapend

@snap[east mr5]
<pre style="line-height: 2em">
-1e+2
1e-2
-1e-2
.123e2
0.123e+2
-0.123e-2
-.123e+2
-1.123e+2
</pre>
@snapend

---

Numbers
=======

@snap[fragment]
Integer numbers within the range [-(2^53)+1, (2^53)-1] are generally considered interoperable.
@snapend

@snap[mt30 fragment]
Floating point numbers shall support accuracy and precision defined by IEEE 754 binary64 (double precision).
@snapend

---

Strings
=======

```bnf
string: '"' utf8-unicode-chars '"'.
```

@snap[mt30 fragment]
Unicode chars that needs to be escaped:
@snapend

@ul[mt30]
* Quotation mark (").
* Reverse solidus (\\).
* Control characters (U+0000 through U+001F)
@ulend

---

Example: object
===============

```json
{
  "Image": {
	"Width":  800,
	"Height": 600,
	"Title":  "View from 15th Floor",
	"Thumbnail": {
		"Url":    "http://www.example.com/image/481989943",
		"Height": 125,
		"Width":  100
	},
	"Animated" : false,
	"IDs": [116, 943, 234, 38793]
  }
}
```

---

Example: array
==============

```json
[
	{
	   "precision": "zip",
	   "Latitude":  37.7668,
	   "Longitude": -122.3959,
	   "Address":   "",
	   "Zip":       "94107",
	   "Country":   "US"
	},
	{
	   "precision": "zip",
	   "Latitude":  37.371991,
	   "Longitude": -122.026020,
	   "Address":   "",
	   "Zip":       "94085",
	   "Country":   "US"
	}
]
```

---

Grammar
=======

```bnf
bool     : "true" | "false".
null     : "null".
string   : '"' chars '"'.

number   : "-"? int frac? exp?.
int      : "0" | [0-9]+.
frac     : "." [0-9]+.
exp      : ("e"|"E") (-|+)? [0-9]+.

object   : "{" property \*(comma property) "}".
property : string ":" value.

array    : "[" value \*(comma value) "]".
```

+++

Whitespace
==========

JSON has a free-form syntax, meaning all forms of whitespace
serve only to separate tokens in the grammar, and have no
semantic significance.

```bnf
white-space     : space | horizontal-tab | new-line | cr
space           : %x20
horizontal-tab  : %x09
new-line        : %x0A
cr              : %x0D // Carriage-Return
```

---

128-bit integers
================

@snap[mt30 fragment]
[JSON specification](https://tools.ietf.org/html/rfc8259) do not define
the precision for integer numbers.
@snapend

@snap[mt30 fragment]
While most implementation of JSON handles number as 64-bit floating-point,
[jsondata](https://github.com/bnclabs/jsondata), a rust implementation
to process JSON, support serialization and de-serialisation of signed
128-bit integers.
@snapend

---

Numbers: Deferred conversion
============================

@snap[mt30 fragment]
When used in context of a document-database, parsing JSON data is a
repeated operation, and many times only specific fields within
a JSON document need to be lookedup.
@snapend

@snap[mt30 fragment]
To that end, [jsondata](https://github.com/bnclabs/jsondata) supports
deferred conversion of numbers.
@snapend

+++

Benchmark results
=================

```rust
Without deferred conversion:

    test json_test::bench_no_deferred    1,093 ns/iter (+/- 88)
    test json_test::bench_num              109 ns/iter (+/- 19)

With deferred conversion:

    test json_test::bench_deferred       1,004 ns/iter (+/- 215)
    test json_test::bench_num               77 ns/iter (+/- 4)
```

An improvement around 10-30%.

```rust
// bench_no_deferred uses the following sample
r#"[10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
// bench_num uses the following sample
"123121.2234234"
```

---

JSON Pointer
============

[JSON Pointer][jptr] defines a string syntax for identifying a specific value
within a JSON document.

@snap[fragment]
A JSON pointer text is made of path components separated by @color[blue](/).
@snapend

<div class="fragment mt30">
For Example: @color[blue](/users/0/name)

<table style="margin: unset" class="mt30">
<tr> <th> path </th> <th> meaning </th> </tr>
<tr> <td>/users</td> <td> fetch the member value with matching key - @color[blue](users) </td> </tr>
<tr> <td>/0    </td> <td> fetch the @color[blue](first value) from an array </td> </tr>
<tr> <td>/name </td> <td> fetch the member value with matching key - @color[blue](name) </td> </tr>
</table>

</div>


[jptr]: https://tools.ietf.org/html/rfc6901

+++

Path matching
=============

@css[text-bold fragment](String-escape)

@snap[fragment]
All instances of quotation mark '"' (%x22), reverse solidus '\' (%x5C),
and control (%x00-1F) characters MUST be un-escaped.
@snapend

@css[text-bold fragment](Tilde-escape)

@snap[fragment]
All instances of ~0 and ~1 must be un-escaped to ~ and /.
@snapend

@css[text-bold fragment](Comparison)

@ul
* If current reference value is an object, path-fragment is compared byte-by-byte with the property name.
* If currently referenced value is a JSON array, path-fragment is interpreted as base-10 integer in ASCII and converted to zero-based index.
@ulend

+++

<!-- .slide: class="jptr-result" -->

Example: Result
===============

@img[west jptr-eg span-45](./template/jpt-eg.png)

<table class="east jptr-eg span-45">
    <tr><th>pointer-text</th> <th>result</th></tr>
    <tr><td>""</td>           <td>// the whole document</td></tr>
    <tr><td>"/foo"</td>       <td>["bar", "baz"]</td></tr>
    <tr><td>"/foo/0"</td>     <td>"bar"</td></tr>
    <tr><td>"/"</td>          <td>0</td></tr>
    <tr><td>"/a~1b"</td>      <td>1</td></tr>
    <tr><td>"/c%d"</td>       <td>2</td></tr>
    <tr><td>"/e^f"</td>       <td>3</td></tr>
    <tr><td>"/g|h"</td>       <td>4</td></tr>
    <tr><td>"/i\\j"</td>      <td>5</td></tr>
    <tr><td>"/k\"l"</td>      <td>6</td></tr>
    <tr><td>"/ "</td>         <td>7</td></tr>
    <tr><td>"/m~0n"</td>      <td>8</td></tr>
</table>

+++

Grammar
=======

<br/>

```bnf
json-pointer    = *( "/" reference-token )
reference-token = *( unescaped | escaped )
unescaped       = %x00-2E | %x30-7D | %x7F-10FFFF
escaped         = "~" ( "0" | "1" )
```

@ul[mt30]
* %x2F ('/') and %x7E ('~') are excluded from 'unescaped'
* @color[blue](~0) means @color[blue](~)
* @color[blue](~1) means @color[blue](/).
* @color[blue](~) escaping is not applied recursively. @color[blue](~01) literally becomes @color[blue](~1).
@ulend

---

@snap[midpoint]
<h1>Thank you</h1>
@snapend
