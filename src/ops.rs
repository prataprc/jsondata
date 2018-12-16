use std::ops::{Neg, Not, Mul, Div, Rem, Add, Sub, Shr, Shl};
use std::ops::{BitAnd, BitXor, BitOr};

use kv::{self, KeyValue};
use json::Json;

impl Neg for Json {
    type Output=Json;

    fn neg(self) -> Json {
        match self {
            Json::Integer(n) => Json::Integer(-n),
            Json::Float(n) => Json::Float(-n),
            _ => Json::Null,
        }
    }
}

impl Not for Json {
    type Output=Json;

    fn not(self) -> Json {
        match self {
            Json::Bool(val) => Json::Bool(!val),
            _ => Json::Null,
        }
    }
}

impl Mul for Json {
    type Output=Json;

    fn mul(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,Object, String as S};

        match (self, rhs) {
            (Integer(l), Integer(r)) => Integer(l*r),
            (Integer(l), Float(r)) => Float((l as f64) * r),
            (lhs@Integer(_), rhs) => rhs.mul(lhs),
            (Float(l), Float(r)) => Float(l*r),
            (Float(l), Integer(r)) => Float(l*(r as f64)),
            (lhs@Float(_), rhs) => rhs.mul(lhs),
            (S(_), Integer(0)) => Null,
            (S(s), Integer(n)) => S(s.repeat(n as usize)),
            (Object(this), Object(other)) => {
                let mut obj = Vec::new();
                obj = mixin_object(obj, this.to_vec());
                obj = mixin_object(obj, other.to_vec());
                Json::Object(obj)
            },
            (_, _) => Null,
        }
    }
}

impl Div for Json {
    type Output=Json;

    fn div(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,String as S};

        match (self, rhs) {
            (Integer(_), Integer(0)) => Null,
            (Integer(_), Float(f)) if f == 0_f64 => Null,
            (Float(_), Integer(0)) => Null,
            (Float(_), Float(f)) if f == 0_f64 => Null,
            (Integer(l), Integer(r)) => Float((l as f64)/(r as f64)),
            (Integer(l), Float(r)) => Float((l as f64)/r),
            (Float(l), Float(r)) => Float(l/r),
            (Float(l), Integer(r)) => Float(l/(r as f64)),
            (S(s), S(patt)) => {
                let arr = s.split(&patt).map(|s| S(s.to_string())).collect();
                Json::Array(arr)
            },
            (_, _) => Null,
        }
    }
}

impl Rem for Json {
    type Output=Json;

    fn rem(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float};

        match (self, rhs) {
            (Integer(_), Integer(0)) => Null,
            (Integer(l), Integer(r)) => Integer(l%r),
            (Integer(_), Float(f)) if f == 0_f64 => Null,
            (Integer(l), Float(r)) => Float((l as f64)%r),
            (Float(_), Integer(0)) => Null,
            (Float(l), Integer(r)) => Float(l%(r as f64)),
            (Float(_), Float(f)) if f == 0_f64 => Null,
            (Float(l), Float(r)) => Float(l%r),
            (_, _) => Null,
        }
    }
}

impl Add for Json {
    type Output=Json;

    fn add(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,Array,Object, String as S};

        match (self, rhs) {
            (Integer(l), Integer(r)) => Integer(l+r),
            (Integer(l), Float(r)) => Float((l as f64)+r),
            (lhs@Integer(_), rhs) => rhs.add(lhs),
            (Float(l), Float(r)) => Float(l+r),
            (Float(l), Integer(r)) => Float(l+(r as f64)),
            (lhs@Float(_), rhs) => rhs.add(lhs),
            (S(l), S(r)) => {
                let mut s = String::new(); s.push_str(&l); s.push_str(&r);
                S(s)
            }
            (Array(l), Array(r)) => {
                let mut a = vec![];
                a.extend_from_slice(&l);
                a.extend_from_slice(&r);
                Array(a)
            }
            (Object(l), Object(r)) => {
                let mut obj = Json::Object(Vec::new());
                l.to_vec().into_iter().for_each(|p| Json::insert(&mut obj, p));
                r.to_vec().into_iter().for_each(|p| Json::insert(&mut obj, p));
                obj
            }
            (_, _) => Null,
        }
    }
}

impl Sub for Json {
    type Output=Json;

    fn sub(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,Array};

        match (self, rhs) {
            (Integer(l), Integer(r)) => Integer(l-r),
            (Integer(l), Float(r)) => Float((l as f64)-r),
            (lhs@Integer(_), rhs) => rhs.sub(lhs),
            (Float(l), Float(r)) => Float(l-r),
            (Float(l), Integer(r)) => Float(l-(r as f64)),
            (lhs@Float(_), rhs) => rhs.sub(lhs),
            (Array(mut lhs), Array(rhs)) => {
                rhs.iter().for_each(|x| {lhs.remove_item(x);});
                Array(lhs)
            },
            (_, _) => Null,
        }
    }
}

impl Shr for Json {
    type Output=Json;

    fn shr(self, rhs: Json) -> Json {
        match (self, rhs) {
            (Json::Integer(l), Json::Integer(r)) => Json::Integer(l>>r),
            (_, _) => Json::Null,
        }
    }
}

impl Shl for Json {
    type Output=Json;

    fn shl(self, rhs: Json) -> Json {
        match (self, rhs) {
            (Json::Integer(l), Json::Integer(r)) => Json::Integer(l<<r),
            (_, _) => Json::Null,
        }
    }
}

impl BitAnd for Json {
    type Output=Json;

    fn bitand(self, rhs: Json) -> Json {
        match (self, rhs) {
            (Json::Integer(l), Json::Integer(r)) => Json::Integer(l&r),
            (_, _) => Json::Null,
        }
    }
}

impl BitXor for Json {
    type Output=Json;

    fn bitxor(self, rhs: Json) -> Json {
        match (self, rhs) {
            (Json::Integer(l), Json::Integer(r)) => Json::Integer(l^r),
            (_, _) => Json::Null,
        }
    }
}

impl BitOr for Json {
    type Output=Json;

    fn bitor(self, rhs: Json) -> Json {
        match (self, rhs) {
            (Json::Integer(l), Json::Integer(r)) => Json::Integer(l|r),
            (_, _) => Json::Null,
        }
    }
}

// TODO: To handle || and && short-circuiting operations.
//impl And for Json {
//    type Output=Json;
//
//    fn and(self, other: Json) -> Self::Output {
//        let lhs = bool::from(self);
//        let rhs = bool::from(other);
//        Json::Bool(lhs & rhs)
//    }
//}
//
//impl Or for Json {
//    type Output=Json;
//
//    fn or(self, other: Json) -> Self::Output {
//        let lhs = bool::from(self);
//        let rhs = bool::from(other);
//        Json::Bool(lhs | rhs)
//    }
//}

fn mixin_object(mut this: Vec<KeyValue>, other: Vec<KeyValue>)
    -> Vec<KeyValue>
{
    use json::Json::{Object};

    for o in other.into_iter() {
        match kv::search_by_key(&this, o.key_ref()) {
            Ok(off) => match (this[off].clone().value(), o.clone().value()) {
                (Object(val), Object(val2)) => {
                    this[off].set_value(Object(mixin_object(val, val2)))
                },
                _ => this.insert(off, o)
            },
            Err(off) => this.insert(off, o.clone()),
        }
    }
    this
}

