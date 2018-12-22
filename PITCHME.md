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
@snap[fragment]
The data exchange standard for web.
@snapend

@snap[mt30 fragment]
Inter-operability with several languages.
@snapend

@snap[mt30 fragment]
Human friendly as apposed to machine friendly.
@snapend

---

Primary types
=============

There are 4 primitive types.

@ul
* **null**, equivalent of None, nil, null in many languages.
* **number**, base 10 representation.
* **bool**, true or false. Both represented in lowercase.
* **string**, double quoted, utf8 encoded plain text.
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

Structured types
================

@snap[west arraytype]
**Array**
@snapend

@snap[east objecttype]
**Object**
@snapend

+++

Examples: object
================

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

+++

Examples: array
===============

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

@snap[midpoint]
<h1>Thank you</h1>
@snapend
