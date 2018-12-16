use json::*;
use test::Bencher;

#[test]
fn test_simple_jsons() {
    use self::Json::{Null, Bool, String, Integer, Float, Array, Object};

    let jsons = include!("../testdata/test_simple.jsons");
    let mut refs = include!("../testdata/test_simple.jsons.ref");
    let refs_len = refs.len();
    let mut jsonbuf = JsonBuf::new();

    let mut n = 4;
    let obj = Vec::new();
    refs[refs_len - n] = Object(obj);
    n -= 1;

    let mut obj = Vec::new();
    let (k, v) = ("key1".to_string(), r#""value1""#.parse().unwrap());
    obj.insert(0, Property::new(k, v));
    refs[refs_len - n] = Object(obj);
    n -= 1;

    let mut obj = Vec::new();
    let (k, v) = ("key1".to_string(), r#""value1""#.parse().unwrap());
    obj.insert(0, Property::new(k, v));
    let (k, v) = ("key2".to_string(), r#""value2""#.parse().unwrap());
    obj.insert(1, Property::new(k, v));
    refs[refs_len - n] = Object(obj);
    n -= 1;

    let mut obj = Vec::new();
    let (k, v) = ("a".to_string(), "1".parse().unwrap());
    obj.insert(0, Property::new(k, v));
    let (k, v) = ("b".to_string(), "1".parse().unwrap());
    obj.insert(1, Property::new(k, v));
    let (k, v) = ("c".to_string(), "1".parse().unwrap());
    obj.insert(2, Property::new(k, v));
    let (k, v) = ("d".to_string(), "1".parse().unwrap());
    obj.insert(3, Property::new(k, v));
    let (k, v) = ("e".to_string(), "1".parse().unwrap());
    obj.insert(4, Property::new(k, v));
    let (k, v) = ("f".to_string(), "1".parse().unwrap());
    obj.insert(5, Property::new(k, v));
    let (k, v) = ("x".to_string(), "1".parse().unwrap());
    obj.insert(6, Property::new(k, v));
    let (k, v) = ("z".to_string(), "1".parse().unwrap());
    obj.insert(7, Property::new(k, v));
    refs[refs_len - n] = Object(obj);

    jsonbuf.set(jsons[51]);
    let value = jsonbuf.parse().unwrap();
    assert_eq!(value, refs[51]);

    let ref_jsons = include!("../testdata/test_simple.jsons.ref.jsons");
    for (i, r) in refs.iter().enumerate() {
        let s = format!("{}", r);
        //println!("{} {}", i, &s);
        assert_eq!(&s, ref_jsons[i], "testcase: {}", i);
    }
}

#[test]
fn test_json_iter() {
    use self::Json::{Integer, Float, Bool, Array, Object, String as S};

    let docs = r#"null 10 10.2 "hello world" true false [1,2] {"a":10}"#;
    let docs: &[u8] = docs.as_ref();
    let mut iter = JsonBuf::iter(docs);
    assert_eq!(Some(Json::Null), iter.next());
    assert_eq!(Some(Integer(10)), iter.next());
    assert_eq!(Some(Float(10.2)), iter.next());
    assert_eq!(Some(S("hello world".to_string())), iter.next());
    assert_eq!(Some(Bool(true)), iter.next());
    assert_eq!(Some(Bool(false)), iter.next());
    assert_eq!(
        Some(Array(vec![Integer(1), Integer(2), ])),
        iter.next()
    );
    assert_eq!(
        Some(Object(vec![Property::new("a".to_string(), Integer(10))])),
        iter.next(),
    );
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
    b.iter(|| {"10.2".parse::<Json>().unwrap()});
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
fn bench_map_nom(b: &mut Bencher) {
    let s = r#"  { "a": 42, "b": ["x","y",12 ] , "c": {"hello":"world"}} "#;
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
