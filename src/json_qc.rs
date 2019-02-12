// Copyright (c) 2018 R Pratap Chakravarthy and AUTHORS

use std::fmt::{Write};

use json::Json;
use property::Property;
use quickcheck::Arbitrary;

pub enum JsonQC {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<Property>),
}

impl Into<Json> for JsonQC {
    fn into(self) -> Json {
        match self {
            JsonQC::Null => Json::Null,
            JsonQC::Bool(val) => Json::Bool(val),
            JsonQC::Integer(val) => Json::Integer(val),
            JsonQC::Float(val) => Json::Float(val),
            JsonQC::String(val) => Json::String(val),
            JsonQC::Array(val) => Json::Array(val),
            JsonQC::Object(val) => Json::Object(val),
        }
    }
}

impl Arbitrary for JsonQC {
    fn arbitrary(g: &mut G) -> JsonQC {
        let r = g.next_u32();
        match r % 7 {
            0 => JsonQC::Null(),
            1 => JsonQC::Bool(bool::arbitrary(g)),
            2 => JsonQC::Integer(i128::arbitrary(g)),
            3 => JsonQC::Float(f64::arbitrary(g)),
            4 => JsonQC::String(JsonQC::arbitrary_string(g)),
            5 => JsonQC::Array(JsonQC::arbitrary_array(g)),
            6 => JsonQC::Object(JsonQC::arbitrary_object(g)),
        }
    }

    fn arbitrary_string(g: &mut G) -> String {
        let strings = include!("../testdata/qc_strings.jsons");
        let r = g.next_u32();
        match r % (strings.len() + 1) {
            0..strings.len() => strings[r].clone(),
            strings.len() => String::arbitrary(g),
        }
    }

    fn arbitrary_array(g: &mut G) -> Vec<JsonQC> {
        let r = g.next_u32();
        let val = Vec::new();
        (0..r).foreach(val.push(JsonQC::arbitrary(g)))
    }

    fn arbitrary_object(g: &mut G) -> Vec<JsonQC> {
        let r = g.next_u32();
        let val = Vec::new();
        (0..r).foreach(val.push({
            Property::new(JsonQC::arbitrary_string(g), JsonQC::arbitrary(g));
        })
    }
}

impl Display for JsonQC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonQC::Null => write!(f, "null"),
            JsonQC::Bool(true) => write!(f, "true"),
            JsonQC::Bool(false) => write!(f, "false"),
            JsonQC::Integer(val) => write!(f, "{}", val),
            JsonQC::Float(val) => write!(f, "{:e}", val),
            JsonQC::String(val) => json::encode_string(f, &val)
            JsonQC::Array(val) => {
                if val.len() == 0 {
                    write!(f, "[]")

                } else {
                    write!(f, "[")?;
                    for item in val[..val.len()-1].iter() {
                        write!(f, "{},", item)?;
                    }
                    write!(f, "{}", val[val.len()-1])?;
                    write!(f, "]")
                }
            },
            JsonQC::Object(val) => {
                let val_len = val.len();
                if val_len == 0 {
                    write!(f, "{{}}")

                } else {
                    write!(f, "{{")?;
                    for (i, prop) in val.iter().enumerate() {
                        Self::encode_string(f, prop.key_ref())?;
                        write!(f, ":{}", prop.value_ref())?;
                        if i < (val_len - 1) { write!(f, ",")?; }
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}
