//! Using jsondata macros

use std::convert::TryInto;

use jsondata::{Json, JsonSerialize};

#[derive(JsonSerialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
struct Parent {
    #[json(try_into = "i128")]
    field1: u8,
    #[json(from_str)]
    field2: i8,
    #[json(to_string)]
    field3: u16,
    field4: i16,
    field5: u32,
    field6: i32,
    field7: u64,
    field8: i64,
    field9: u128,
    field10: i128,
    field11: bool,
    field12: usize,
    field13: isize,
    field14: String,
    field15: Vec<u8>,
    field16: Child,
}

#[derive(JsonSerialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
struct Child {
    fIeld1: i128,
    another_fieldWithTuple: (String, i128),
}

#[derive(JsonSerialize, Default, Clone, Debug)]
#[allow(non_snake_case)]
struct Floats {
    field1: f32,
    field2: f64,
}

fn main() {
    let p_ref = {
        let c_ref = Child {
            fIeld1: 10,
            another_fieldWithTuple: ("hello".to_string(), 10000000),
        };
        Parent {
            field1: 10,
            field2: -10,
            field3: 100,
            field4: -100,
            field5: 1000,
            field6: -1000,
            field7: 10000,
            field8: -10000,
            field9: 1000000,
            field10: -1000000,
            field11: true,
            field12: 100,
            field13: 102,
            field14: "hello world".to_string(),
            field15: vec![1, 2, 3, 4],
            field16: c_ref,
        }
    };

    let ref_s = concat!(
        r#"{"field1":10,"field10":-1000000,"#,
        r#""field11":true,"field12":100,"field13":102,"field14":"hello world","#,
        r#""field15":[1,2,3,4],"#,
        r#""field16":{"another_fieldwithtuple":["hello",10000000],"field1":10},"#,
        r#""field2":"-10","field3":"100","#,
        r#""field4":-100,"field5":1000,"field6":-1000,"field7":10000,"#,
        r#""field8":-10000,"field9":1000000}"#
    );
    let jval: Json = p_ref.clone().into();
    let p: Parent = jval.clone().try_into().unwrap();
    assert_eq!(jval.to_string(), ref_s);
    assert_eq!(p, p_ref);

    println!("{}", jval.to_string());

    let f_ref = Floats {
        field1: 10.2345678,
        field2: -10.12312312312311,
    };

    let ref_s = r#"{"field1":1.0234567642211914e1,"field2":-1.012312312312311e1}"#;

    let jval: Json = f_ref.clone().try_into().unwrap();
    let f: Floats = jval.clone().try_into().unwrap();

    assert_eq!(jval.to_string(), ref_s);
    assert!(f.field1 == f_ref.field1);
    assert!(f.field2 == f_ref.field2);

    println!("{}", jval.to_string());
}
