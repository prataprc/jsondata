//! Using jsondata macros

use std::convert::TryInto;

use jsondata::{Json, JsonData};

//#[derive(JsonData)]
//struct Parent {
//    field1: u8,
//    field2: i8,
//    field3: u16,
//    field4: i16,
//    field5: u32,
//    field6: i32,
//    field7: u64,
//    field8: i64,
//    field9: u128,
//    field10: i128,
//    field11: bool,
//    field12: f32,
//    field13: f64,
//    field14: usize,
//    field15: isize,
//    field16: String,
//    field17: Vec<u8>,
//    field18: Child,
//}

#[derive(JsonData, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Child {
    field1: i128,
}

fn main() {
    let c_ref = Child { field1: 10 };
    let jval: Json = c_ref.clone().try_into().unwrap();
    let c: Child = jval.clone().try_into().unwrap();

    println!("{}", jval.to_string());
    assert_eq!(c, c_ref);
}
