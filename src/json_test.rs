use std::f64;
use std::fmt::Write;
use std::fs::File;

use json::{Json, Jsons};
use num::{Floating, Integral};
use property::Property;
use test::Bencher;

#[test]
fn test_json_constructor() {
    use self::Json;

    assert_eq!(Json::new(10), Json::Integer(Integral::new("10")));
}

#[test]
fn test_simple_jsons() {
    use self::Json::{Array, Bool, Null, Object, String};

    let jsons = include!("../testdata/test_simple.jsons");
    let refs = include!("../testdata/test_simple.jsons.ref");

    for (i, json) in jsons.iter().enumerate() {
        let mut value: Json = json.parse().unwrap();
        value.compute().unwrap();
        assert_eq!(value, refs[i], "testcase {}", i);
    }
}

#[test]
fn test_simple_jsons_ref() {
    use self::Json::{Array, Bool, Null, Object, String};

    let jsons = include!("../testdata/test_simple.jsons");
    let refs = include!("../testdata/test_simple.jsons.ref");

    let value: Json = jsons[51].parse().unwrap();
    assert_eq!(value, refs[51]);

    let ref_jsons = include!("../testdata/test_simple.jsons.ref.jsons");
    for (i, r) in refs.iter().enumerate() {
        let s = format!("{}", r);
        //println!("{} {}", i, &s);
        assert_eq!(&s, ref_jsons[i], "testcase: {}", i);
    }
}

#[test]
fn test_convert() {
    let js: Json = true.into();
    assert_eq!(js, Json::new(true));

    let js: Json = 1024.into();
    assert_eq!(js, Json::new(1024));

    let js: Json = 1024.2.into();
    assert_eq!(js, Json::new(1024.2));

    let js: Json = "hello world".to_string().into();
    assert_eq!(js, Json::new("hello world"));

    let js: Json = "hello world".into();
    assert_eq!(js, Json::new("hello world"));
}

#[test]
fn test_deferred() {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    let value: Json = inp.parse().unwrap();
    let refval = Json::Array(vec![
        Json::Float(Floating::new("10123.1231")),
        Json::Float(Floating::new("1231.123123")),
        Json::Float(Floating::new("1233.123123")),
        Json::Float(Floating::new("123.1231231")),
        Json::Float(Floating::new("12312e10")),
    ]);
    assert_eq!(value, refval);
}

#[test]
fn test_validate_sorted() {
    let json = r#"{"z":1,"a":[2, {"x":"y"}, true],"c":[null],"d":3}"#;
    let mut value: Json = json.parse().unwrap();

    assert_eq!(value.validate(), Ok(()));

    let mut props: Vec<Property> = Vec::new();
    let prop = vec![Property::new("x", Json::new("y"))];
    let items = vec![Json::new(2), Json::new(prop), Json::new(true)];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push(Property::new("z", Json::new(1)));

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_compute() {
    let json = r#"{"z":1,"a":[2, {"x":"y"}, true],"c":[null],"d":3}"#;
    let mut value: Json = json.parse().unwrap();

    assert_eq!(value.compute(), Ok(()));

    let mut props: Vec<Property> = Vec::new();
    let prop = vec![Property::new("x", Json::new("y"))];
    let items = vec![Json::new(2), Json::new(prop), Json::new(true)];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push(Property::new("z", Json::new(1)));

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_json5_whitespace() {
    let text = "\u{0009} \u{000a} \u{000b} \u{000c} ".to_string()
        + &("\u{00a0} \r \t \n 0x1234".to_string());
    let json: Json = text.parse().unwrap();
    assert_eq!(json.integer(), Json::new(0x1234).integer());
}

#[test]
fn test_json5_num() {
    let mut json: Json = "0x1234".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json, Json::new(0x1234));

    let mut json: Json = "1234.".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json.float(), Json::new(1234.0).float());

    let mut json: Json = ".1234".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json, Json::new(0.1234));

    let mut json: Json = ".1234.".parse().unwrap();
    json.compute().unwrap_err();
    assert_eq!(json.float(), None);

    let mut json: Json = "[Infinity, -Infinity, NaN]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Json::new(f64::INFINITY),
        Json::new(f64::NEG_INFINITY),
        Json::new(f64::NAN),
    ]);
    assert_eq!(json, value);

    let mut json: Json = " [ 0xdecaf, -0xC0FFEE ]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![Json::new(0xdecaf), Json::new(-0xC0_FFEE)]);
    assert_eq!(json, value);

    let mut json: Json = "[ 123, 123.456, .456, 123e-456 ]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Json::new(123),
        Json::new(123.456),
        Json::new(0.456),
        Json::new(123e-456),
    ]);
    assert_eq!(json, value);
}

#[test]
fn test_json5_array() {
    let json: Json = "[]".parse().unwrap();
    let value = Json::new::<Vec<Json>>(vec![]);
    assert_eq!(json, value);

    let mut json: Json = r#"[ 1, true, "three", ]"#.parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![Json::new(1), Json::new(true), Json::new("three")]);
    assert_eq!(json, value);

    let json: Json = r#"[ [1, true, "three"], [4, "five", 0x6], ]"#.parse().unwrap();
    let value = Json::new(vec![
        Json::new(vec![Json::new(1), Json::new(true), Json::new("three")]),
        Json::new(vec![Json::new(4), Json::new("five"), Json::new(0x6)]),
    ]);
    assert_eq!(json, value);
}

#[test]
fn test_json5_object() {
    let json: Json = "{}".parse().unwrap();
    let value = Json::new::<Vec<Property>>(vec![]);
    assert_eq!(json, value);

    let mut json: Json = "{ width: 1920, height: 1080, }".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Property::new("height", 1080.into()),
        Property::new("width", 1920.into()),
    ]);
    assert_eq!(json, value);

    let mut json: Json = r#"{ image: { width: 1920, height: 1080, "aspect-ratio": "16:9", } }"#
        .parse()
        .unwrap();
    json.compute().unwrap();
    let props = Json::new(vec![
        Property::new("aspect-ratio", "16:9".into()),
        Property::new("height", 1080.into()),
        Property::new("width", 1920.into()),
    ]);
    let value = Json::new(vec![Property::new("image", props)]);
    assert_eq!(json, value);

    let mut json: Json = r#"[ { name: "Joe", age: 27 }, { name: "Jane", age: 32 }, ]"#
        .parse()
        .unwrap();
    json.compute().unwrap();
    let obj1 = Json::new::<Vec<Property>>(vec![
        Property::new("age", 27.into()),
        Property::new("name", "joe".into()),
    ]);
    let obj2 = Json::new::<Vec<Property>>(vec![
        Property::new("age", 32.into()),
        Property::new("name", "jane".into()),
    ]);
    let value = Json::new(vec![obj1, obj2]);
    assert_eq!(json, value);
}

#[test]
fn test_stream() {
    //let mut js: Jsons<&[u8]> = b"".as_ref().into();
    //assert!(js.next().is_none());

    //let mut js: Jsons<&[u8]> = b" \t \r \n ".as_ref().into();
    //assert!(js.next().is_none());

    //let mut js: Jsons<&[u8]> = b" 1".as_ref().into();
    //assert_eq!(js.next().unwrap().unwrap(), Json::new(1));

    let file = File::open("testdata/stream.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(js.next().unwrap().unwrap(), Json::new(1));

    assert_eq!(js.next().unwrap().unwrap(), Json::Null);
    assert_eq!(js.next().unwrap().unwrap(), Json::new(true));
    assert_eq!(js.next().unwrap().unwrap(), Json::new(false));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(102));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(10.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(0));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(100));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(1));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(2.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.02));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(20.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(20.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(200.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(2.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-102));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-10.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-0));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-100));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-001));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-00.00));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-2.00));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.02));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-20.0));

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("hello\"  \r\t".to_string())
    );

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("helloȴ\\ 𝄞".to_string())
    );

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("\'é\' character is one Unicode code point é while \'é\' e\u{301} ".to_string())
    );

    assert_eq!(js.next().unwrap().unwrap(), Json::new::<Vec<Json>>(vec![]));
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![Json::new(10)])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null, true.into(), false.into(), 10.into(), "tru\"e".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(), "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null, true.into(), false.into(),
            "hello\" \\ / \u{8} \u{c}\n\r\t".into()
        ])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            102.into(), 10.2.into(), 0.2.into(), 0.into(),
            "helloȴ\\ 𝄞".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            100.into(), 1.into(), 0.0.into(), 2.0.into(),
            "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            0.2.into(), 0.02.into(), 0.0.into(), 0.2.into(), 0.2.into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            (-102).into(), (-100).into(), (-0.0).into(), (-20.0).into(),
        ])
    );

    assert_eq!(js.next().unwrap().unwrap(), Json::new::<Vec<Property>>(vec![]));
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![ Property::new("key1", "value1".into()) ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Property::new("key1", "value1".into()),
            Property::new("key2", "value2".into()),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Property::new("z", 1.into()),
            Property::new("a", 1.into()),
            Property::new("c", 1.into()),
            Property::new("d", 1.into()),
            Property::new("f", 1.into()),
            Property::new("e", 1.into()),
            Property::new("b", 1.into()),
            Property::new("x", 1.into()),
        ])
    );
}

#[bench]
fn bench_null(b: &mut Bencher) {
    b.iter(|| "null".parse::<Json>().unwrap());
}

#[bench]
fn bench_bool(b: &mut Bencher) {
    b.iter(|| "false".parse::<Json>().unwrap());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| "123121.2234234".parse::<Json>().unwrap());
}

#[bench]
fn bench_hexnum(b: &mut Bencher) {
    b.iter(|| "0x1235abcd".parse::<Json>().unwrap());
}

#[bench]
fn bench_string(b: &mut Bencher) {
    let s = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_array(b: &mut Bencher) {
    let s = r#" [null,true,false,10,"tru\"e"]"#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_map(b: &mut Bencher) {
    let s = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_null_to_json(b: &mut Bencher) {
    let val = "null".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_bool_to_json(b: &mut Bencher) {
    let val = "false".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_num_to_json(b: &mut Bencher) {
    let val = "10.2".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_string_to_json(b: &mut Bencher) {
    let inp = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_array_to_json(b: &mut Bencher) {
    let inp = r#" [null,true,false,10,"tru\"e"]"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_map_to_json(b: &mut Bencher) {
    let inp = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| inp.parse::<Json>().unwrap());
}

#[bench]
fn bench_no_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| inp.parse::<Json>().unwrap().compute().unwrap());
}

#[bench]
fn bench_json5_num(b: &mut Bencher) {
    let inp = r#" -Infinity"#;
    b.iter(|| inp.parse::<Json>().unwrap().compute().unwrap());
}
