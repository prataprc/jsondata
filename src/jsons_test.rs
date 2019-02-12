use std::fs::File;

use crate::json::Json;
use crate::jsons::Jsons;
use crate::property::Property;

#[test]
fn test_stream0() {
    let mut js: Jsons<&[u8]> = b"".as_ref().into();
    assert!(js.next().is_none());

    let mut js: Jsons<&[u8]> = b" \t \r \n ".as_ref().into();
    assert!(js.next().is_none());

    let mut js: Jsons<&[u8]> = b" 1".as_ref().into();
    assert_eq!(js.next().unwrap().unwrap(), Json::new(1));

    let mut js: Jsons<&[u8]> = b" n".as_ref().into();
    let value = js.next().unwrap().unwrap();
    assert!(value.is_error());
    assert_eq!(
        value.error().unwrap(),
        "parse: expected null at offset:0 line:1 col:1".to_string()
    );
}

#[test]
fn test_stream1() {
    let file = File::open("testdata/stream1.jsons").unwrap();
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
}

#[test]
fn test_stream11() {
    let file = File::open("testdata/stream11.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(2.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-102));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-10.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-0));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-100));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-1));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-00.00));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-2.00));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.02));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-20.0));
}

#[test]
fn test_stream2() {
    let file = File::open("testdata/stream2.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

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
    assert_eq!(js.next().unwrap().unwrap(), Json::new(vec![Json::new(10)]));
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null,
            true.into(),
            false.into(),
            10.into(),
            "tru\"e".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
    );
}

#[test]
fn test_stream3() {
    let file = File::open("testdata/stream3.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null,
            true.into(),
            false.into(),
            "hello\" \\ / \u{8} \u{c}\n\r\t".into()
        ])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            102.into(),
            10.2.into(),
            0.2.into(),
            0.into(),
            "helloȴ\\ 𝄞".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            100.into(),
            1.into(),
            0.0.into(),
            2.0.into(),
            "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            0.2.into(),
            0.02.into(),
            0.0.into(),
            0.2.into(),
            0.2.into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            (-102).into(),
            (-100).into(),
            (-0.0).into(),
            (-20.0).into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Property>>(vec![])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![Property::new("key1", "value1".into())])
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

    let obj = Json::new(vec![Property::new("key3", 20.into())]);
    let obj = Json::new(vec![Property::new("key2", obj)]);
    let arr = Json::new::<Vec<Json>>(vec!["world".into(), obj]);
    let obj = Json::new(vec![Property::new("key1", arr)]);
    let arr = Json::new::<Vec<Json>>(vec!["hello".into(), obj]);
    assert_eq!(js.next().unwrap().unwrap(), arr);
}
