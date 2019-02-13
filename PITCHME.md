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
Human friendly as opposed to machine friendly.
@snapend

---

Data types
==========

JSON defines the highest common subset of data types across
sevaral languages.

@snap[fragment mt30]
<b>Primitive types</b>
@snapend

@ul
* **null**, equivalent of None, nil, null in many languages.
* **number**, base 10 representation.
* **bool**, true or false. Both represented in lower-case.
* **string**, double quoted, utf8 encoded plain text.
@ulend

@snap[fragment mt30]
<b>Structured types</b>
@snapend

@ul
* **Array**, heterogeneous collection of other types, with number index.
* **Object**, heterogeneous collection of other types, with string index.
@ulend


+++

Examples
========

@snap[fragment text-center text-12]
"Hello world!"
@snapend

<br/>

@snap[fragment text-center text-12]
42
@snapend

<br/>

@snap[fragment text-center text-12]
true
@snapend

<br/>

@snap[fragment text-center text-12]
null
@snapend

+++

Examples: Numbers
=================

@snap[west ml20 text-12]
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

@snap[east mr20 text-12]
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

@snap[mt30 fragment]
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
	   "Country":   "US"
	},
	{
	   "precision": "zip",
	   "Latitude":  37.371991,
	   "Longitude": -122.026020,
	   "Address":   "",
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

White space
===========

JSON has a free-form syntax, meaning all forms of white space
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
a JSON document need to be looked up.
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
For this example document: <br/>
@css[text-07]({"users": [ {"name": "joe", age: 20}, {"name": "jack", age: 30}, {"name": "jane", age: 25} ]})

<table style="margin: unset; " class="mt30">
<tr> <th> path </th> <th> value </th> </tr>
<tr> <td>/users</td> <td class="text-07"> [ {"name": "joe", age: 20}, {"name": "jack", age: 30}, {"name": "jane", age: 25}  </td> </tr>
<tr> <td>/users/0    </td> <td class="text-07"> {"name": "joe", age: 20}  </td> </tr>
<tr> <td>/users/0/name </td> <td class="text-07"> "joe"  </td> </tr>
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
* %x2F ('/') and %x7E ('~') are excluded from 'un-escaped'
* @color[blue](~0) means @color[blue](~)
* @color[blue](~1) means @color[blue](/).
* @color[blue](~) escaping is not applied recursively. @color[blue](~01) literally becomes @color[blue](~1).
@ulend

---

Json5
=====

Corner cases and good to have features that was left out in JSON.

@ul
* Object keys may be an ECMAScript 5.1 IdentifierName.
* Objects may have a single trailing comma.
* Arrays may have a single trailing comma.
* Strings may be single quoted.
* Strings may span multiple lines by escaping new line characters.
* Strings may include character escapes.
* Numbers may be hexadecimal.
* Numbers may have a leading or trailing decimal point.
* Numbers may be IEEE 754 positive infinity, negative infinity, and NaN.
* Numbers may begin with an explicit plus sign.
* Single and multi-line comments are allowed.
* Additional white space characters are allowed.
@ulend

+++

Identifier for object key
=========================

<br/>

```json
{
    width: 1920,
    height: 1080,
}
```

and

```json
{
    "width": 1920,
    "height": 1080,
}
```

Are equivalent.

+++

Trailing comma
==============

<br/>

```json
[
    { name: 'Joe', age: 27, },
    { name: 'Jane', age: 32 },
]
```

And

```json
[
    { name: 'Joe', age: 27 },
    { name: 'Jane', age: 32 }
]
```

Are equivalent.

+++

Single quoted strings
=====================

```json
{
    image: {
        width: 1920,
        height: 1080,
        'aspect-ratio': '16:9',
    }
}
```

And

```json
{
    image: {
        width: 1920,
        height: 1080,
        "aspect-ratio": "16:9",
    }
}
```

Are equivalent.

+++

Multi-line strings
==================

<br/>

```json
"Lorem ipsum dolor sit amet, \
consectetur adipiscing elit."
```

And

```json
'Lorem ipsum dolor sit amet, consectetur adipiscing elit.'
```

Are equivalent.

+++

<!-- .slide: class="chresc" -->

Character escapes in strings
============================

A string containing only a single reverse solidus character
may be represented as ``\x5C`` or ``\u005C``.

A string containing only the musical score character
🎼 (U+1F3BC) may be represented as '\uD83C\uDFBC'.

Escape | Sequence Description | Code Point
-------|----------------------|-----------
  \'   | Apostrophe	          | U+0027
  \"   | Quotation mark	      | U+0022
  \\   | Reverse solidus      | U+005C
  \b   | Backspace            | U+0008
  \f   | Form feed            | U+000C
  \n   | Line feed            | U+000A
  \r   | Carriage return      | U+000D
  \t   | Horizontal tab       | U+0009
  \v   | Vertical tab         | U+000B
  \0   | Null                 | U+0000

+++

Hexadecimal numbers
===================

<br/>

```json
{
    positiveHex: 0xdecaf,
    negativeHex: -0xC0FFEE,
}
```

And

```json
{
    positiveHex: 912559,
    negativeHex: -12648430,
}
```

Are equivalent.

+++

Leading/Trailing decimal point
==============================

<br/>

```json
{
    integer: 123,
    withFractionPart: 123.456,
    onlyFractionPart: .456,
    trailingPoint: 456.,
    withExponent: 123e-456,
}
```

+++

+/- infinity and NaN
====================

<br/>

```json
{
    positiveInfinity: Infinity,
    negativeInfinity: -Infinity,
    notANumber: NaN,
}
```

+++

+ Numbers
=========

<br/>

```json
{
    impliedPositive: 123,
    explicitPositive: +123,
}
```

+++

Comments
========

<br/>

```json
// This is a single line comment.

/* This is a multi-
   line comment. */
```

+++

<!-- .slide: class="whitespace" -->

White space characters
======================

Additional white space chars:

Code Points          | Description
---------------------|-----------------
 U+0009	             | Horizontal tab
 U+000A	             | Line feed
 U+000B	             | Vertical tab
 U+000C	             | Form feed
 U+000D	             | Carriage return
 U+0020	             | Space
 U+00A0	             | Non-breaking space
 U+2028	             | Line separator
 U+2029	             | Paragraph separator
 U+FEFF	             | Byte order mark
 Unicode Zs category | Any other character in the Space Separator Unicode category

---

Sorting JSON
============

The need to a sort order JSON types and values arise because:

@ul
* Used as data model in many document databases.
* Schema less.
* Building complex indexes on document fields.
@ulend

+++

Sort order for types
====================

@ul
* **Null** type shall sort before all other types.
* **Boolean** type shall sort after Null type.
* **Number** type shall sort after Boolean type.
* **String** type shall sort after Number type.
* **Array** type shall sort after String type.
* **Object** type shall sort after Array type.
@ulend

@css[fragment](Among Boolean type, false value sort before true value.)

+++

Sort order for numerical values
===============================

JSON specification does not define the type and precision for numbers.

* Treat integers and floating point numbers uniformly as same type.
* Integers are capped at i128 bit precision.
* Floating point numbers defined with f64 precision.

When comparing integer number and floating-point number:

* f64 values that are <= -2^127 will sort before all i128 integers.
* f64 values that are >= 2^127-1 will sort after all i128 integers.
* f64 NaN, Not a Number, value shall sort after all i128 integers.
* All other f64 values shall be cast to i128 number and compared.
* -Infinity shall sort before all numbers.
* +Infinity shall sort after all numbers.
* NaN shall sort after +Infinity.

+++

Sort order for string
=====================

String values are not interpreted for code-points are utf8 encoding.
They are just binary compared.

+++

Sort order for array
====================

Array items shall be compared recursively, applying the comparison
logic on each array item and its counterpart. For example:

* [] sort before [null].
* [null] sort before [true].
* [true] sort before [[null]].
* [10,20] sort before [30].
* [10, "hello"] sort before [10, "hello", "world"].

+++

Sort order for object
=====================

* All (key,value) pairs within the object shall be presorted based on the key.
* When comparing two objects, comparison shall start from first key and proceed to the last key.
* If two keys are equal at a given position within the objects, then its corresponding values shall be compared.
* When one object is a subset of another object, as in, if one object contain all the (key,value) properties that the other object has then it shall sort before the other object.

+++

Range operation
===============

* Minbound denote from the beginning.
* Maxbound denote till the end.

---


@snap[midpoint]
<h1>Thank you</h1>
@snapend
