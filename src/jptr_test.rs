use jptr::{quote, unquote};
use json::Json;

#[test]
fn quote_test() {
    let jptr = r#"data/~"\"#;
    let refvalue = r#"data~1~0\"\\"#.to_string();
    assert_eq!(quote(&jptr), refvalue);

    let jptr = r#"\x00\x01\x02\x03\x04\x05\x06"#;
    let refvalue = "\\\\x00\\\\x01\\\\x02\\\\x03\\\\x04\\\\x05\\\\x06".to_string();
    assert_eq!(quote(&jptr), refvalue);

    let jptr = r#"\x07\x08\x09\x0a\x0b\x0c\x0d"#;
    let refvalue = "\\\\x07\\\\x08\\\\x09\\\\x0a\\\\x0b\\\\x0c\\\\x0d".to_string();
    assert_eq!(quote(&jptr), refvalue);

    let jptr = r#"\x0e\x0f"#;
    let refvalue = "\\\\x0e\\\\x0f".to_string();
    assert_eq!(quote(&jptr), refvalue);
}

#[test]
fn unquote_test() {
    let jptr = r#"data/~"\"#.to_string();
    assert_eq!(unquote(&quote(&jptr)).unwrap(), jptr);

    let jptr = r#"\x00\x01\x02\x03\x04\x05\x06"#.to_string();
    assert_eq!(unquote(&quote(&jptr)).unwrap(), jptr);

    let jptr = r#"\x07\x08\x09\x0a\x0b\x0c\x0d"#.to_string();
    assert_eq!(unquote(&quote(&jptr)).unwrap(), jptr);

    let jptr = r#"\x0e\x0f"#.to_string();
    assert_eq!(unquote(&quote(&jptr)).unwrap(), jptr);

    let jptr = "/my/path".to_string();
    assert_eq!(unquote(&jptr).unwrap(), "/my/path".to_string());

    assert_eq!(unquote(r#"/i\\j"#).unwrap(), r#"/i\j"#);
    assert_eq!(unquote(r#"/k\"l"#).unwrap(), r#"/k"l"#);
}


#[test]
fn jptr_test() {
    let text = r#"
       {
          "foo": ["bar", "baz"],
          "": 0,
          "a/b": 1,
          "c%d": 2,
          "e^f": 3,
          "g|h": 4,
          "i\\j": 5,
          "k\"l": 6,
          " ": 7,
          "m~n": 8
       }
    "#;
    let json: Json = text.parse().unwrap();

    assert_eq!(json.get("").unwrap(), json);

    let refv = Json::new(vec![Json::new("bar"), Json::new("baz")]);
    assert_eq!(json.get("/foo").unwrap(), refv);

    assert_eq!(json.get("/foo/0").unwrap(), Json::new("bar"));
    assert_eq!(json.get("/").unwrap(), Json::new(0));
    assert_eq!(json.get("/a~1b").unwrap(), Json::new(1));
    assert_eq!(json.get("/c%d").unwrap(), Json::new(2));
    assert_eq!(json.get("/e^f").unwrap(), Json::new(3));
    assert_eq!(json.get("/g|h").unwrap(), Json::new(4));
    assert_eq!(json.get(r#"/i\\j"#).unwrap(), Json::new(5));
    assert_eq!(json.get(r#"/k\"l"#).unwrap(), Json::new(6));
    assert_eq!(json.get("/ ").unwrap(), Json::new(7));
    assert_eq!(json.get("/m~0n").unwrap(), Json::new(8));
}
