use std::fmt::{Write};

use json::{Json, IntText, FloatText};
use kv::KeyValue;
use test::Bencher;

#[test]
fn test_json_constructor() {
    use self::Json;

    assert_eq!(Json::new(10), Json::Integer(IntText::new("10")));
}

#[test]
fn test_simple_jsons() {
    use self::Json::{Null, Bool, String, Array, Object};

    let jsons = include!("../testdata/test_simple.jsons");
    let refs = include!("../testdata/test_simple.jsons.ref");

    for (i, json) in jsons.iter().enumerate() {
        let mut value: Json = json.parse().unwrap();
        value.compute();
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
        Json::Float(FloatText::new("10123.1231")),
        Json::Float(FloatText::new("1231.123123")),
        Json::Float(FloatText::new("1233.123123")),
        Json::Float(FloatText::new("123.1231231")),
        Json::Float(FloatText::new("12312e10")),
    ]);
    assert_eq!(value, refval);
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
    b.iter(|| {inp.parse::<Json>().unwrap().compute()});
}
