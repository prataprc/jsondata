use std::fmt::{Write};
use std::f64;

use json::{Json};
use num::{Integral, Floating};
use property::Property;
use test::Bencher;

#[test]
fn test_json_constructor() {
    use self::Json;

    assert_eq!(Json::new(10), Json::Integer(Integral::new("10")));
}

#[test]
fn test_simple_jsons() {
    use self::Json::{Null, Bool, String, Array, Object};

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
    use self::Json::{Null, Bool, String, Array, Object};

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
    let items = vec![ Json::new(2), Json::new(prop), Json::new(true) ];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push( Property::new("z", Json::new(1)) );

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_compute() {
    let json = r#"{"z":1,"a":[2, {"x":"y"}, true],"c":[null],"d":3}"#;
    let mut value: Json = json.parse().unwrap();

    assert_eq!(value.compute(), Ok(()));

    let mut props: Vec<Property> = Vec::new();
    let prop = vec![Property::new("x", Json::new("y"))];
    let items = vec![ Json::new(2), Json::new(prop), Json::new(true) ];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push( Property::new("z", Json::new(1)) );

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_json5_num() {
    let json: Json = "0x1234".parse().unwrap();
    assert_eq!(json, Json::new(0x1234));

    let json: Json = "1234.".parse().unwrap();
    assert_eq!(json.float(), Json::new(1234.0).float());

    let json: Json = ".1234".parse().unwrap();
    assert_eq!(json, Json::new(0.1234));

    let json: Json = ".1234.".parse().unwrap();
    assert_eq!(json.float(), None);

    let json: Json = "[Infinity, -Infinity, NaN]".parse().unwrap();
    let value = Json::new(vec![
        Json::new(f64::INFINITY), Json::new(f64::NEG_INFINITY),
        Json::new(f64::NAN)
    ]);
    assert_eq!(json, value);
}

#[test]
fn test_json5_whitespace() {
    let text = "\u{0009} \u{000a} \u{000b} \u{000c} ".to_string() +
        &("\u{00a0} \r \t \n 0x1234".to_string());
    let json: Json = text.parse().unwrap();
    assert_eq!(json.integer(), Json::new(0x1234).integer());
}

#[bench]
fn bench_null(b: &mut Bencher) {
    b.iter(|| {"null".parse::<Json>().unwrap()});
}

#[bench]
fn bench_bool(b: &mut Bencher) {
    b.iter(|| {"false".parse::<Json>().unwrap()});
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| {"123121.2234234".parse::<Json>().unwrap()});
}

#[bench]
fn bench_hexnum(b: &mut Bencher) {
    b.iter(|| {"0x1235abcd".parse::<Json>().unwrap()});
}

#[bench]
fn bench_string(b: &mut Bencher) {
    let s = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    b.iter(|| {s.parse::<Json>().unwrap()});
}

#[bench]
fn bench_array(b: &mut Bencher) {
    let s = r#" [null,true,false,10,"tru\"e"]"#;
    b.iter(|| {s.parse::<Json>().unwrap()});
}

#[bench]
fn bench_map(b: &mut Bencher) {
    let s = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    b.iter(|| {s.parse::<Json>().unwrap()});
}

#[bench]
fn bench_null_to_json(b: &mut Bencher) {
    let val = "null".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_bool_to_json(b: &mut Bencher) {
    let val = "false".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_num_to_json(b: &mut Bencher) {
    let val = "10.2".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_string_to_json(b: &mut Bencher) {
    let inp = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_array_to_json(b: &mut Bencher) {
    let inp = r#" [null,true,false,10,"tru\"e"]"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_map_to_json(b: &mut Bencher) {
    let inp = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {outs.clear(); write!(outs, "{}", val)});
}

#[bench]
fn bench_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| {inp.parse::<Json>().unwrap()});
}

#[bench]
fn bench_no_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| {inp.parse::<Json>().unwrap().compute().unwrap()});
}

#[bench]
fn bench_json5_num(b: &mut Bencher) {
    let inp = r#" -Infinity"#;
    b.iter(|| {inp.parse::<Json>().unwrap().compute().unwrap()});
}
