use std::ops::{Neg, Not, Mul, Div, Rem, Add, Sub, Shr, Shl};
use std::ops::{BitAnd, BitXor, BitOr};

use property::{self, Property};
use json::Json;

impl Neg for Json {
    type Output=Json;

    fn neg(self) -> Json {
        match self {
            Json::Integer(_) => {
                Json::new(-self.integer().unwrap())
            },
            Json::Float(_) => {
                Json::new(-self.float().unwrap())
            },
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

impl Add for Json {
    type Output=Json;

    fn add(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,Array,Object, String as S};

        match (&self, &rhs) {
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l+r)
            },
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l+r)
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64)+r)
            },
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l+(r as f64))
            },
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
                use json;

                let mut obj = Json::Object(Vec::new());
                l.to_vec().into_iter().for_each(|p| json::insert(&mut obj, p));
                r.to_vec().into_iter().for_each(|p| json::insert(&mut obj, p));
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

        match (&self, &rhs) {
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l-r)
            },
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l-r)
            }
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64)-r)
            },
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l-(r as f64))
            },
            (Array(lhs), Array(rhs)) => {
                let mut res = lhs.clone();
                rhs.iter().for_each(|x| {res.remove_item(x);});
                Array(res)
            },
            (_, _) => Null,
        }
    }
}

impl Mul for Json {
    type Output=Json;

    fn mul(self, rhs: Json) -> Json {
        use json::Json::{Null,Integer,Float,Object, String as S};

        match (&self, &rhs) {
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                Json::new(l*r)
            },
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                Json::new((l as f64)*r)
            },
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                Json::new(l*(r as f64))
            },
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                Json::new(l*r)
            },
            (S(s), Integer(_)) => {
                let n = rhs.integer().unwrap();
                if n == 0 { Null } else { S(s.repeat(n as usize)) }
            },
            (Integer(_), S(s)) => {
                let n = self.integer().unwrap();
                if n == 0 { Null } else { S(s.repeat(n as usize)) }
            }
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

        match (&self, &rhs) {
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                if r == 0 { Null } else { Json::new(l/r) }
            },
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                if r == 0_f64 { Null } else { Json::new((l as f64)/r) }
            },
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                if r == 0 { Null } else { Json::new(l/(r as f64)) }
            },
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                if r == 0_f64 { Null } else { Json::new(l/r) }
            },
            (S(s), S(patt)) => {
                let arr = s.split(patt).map(|s| S(s.to_string())).collect();
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

        match (&self, &rhs) {
            (Integer(_), Integer(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.integer().unwrap());
                if r == 0 { Null } else { Json::new(l%r) }
            },
            (Integer(_), Float(_)) => {
                let (l, r) = (self.integer().unwrap(), rhs.float().unwrap());
                if r == 0_f64 { Null } else { Json::new((l as f64)%r) }
            },
            (Float(_), Integer(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.integer().unwrap());
                if r == 0 { Null } else { Json::new(l%(r as f64)) }
            },
            (Float(_), Float(_)) => {
                let (l, r) = (self.float().unwrap(), rhs.float().unwrap());
                if r == 0_f64 { Null } else { Json::new(l%r) }
            },
            (_, _) => Null,
        }
    }
}

impl Shr for Json {
    type Output=Json;

    fn shr(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l>>r),
            (_, _) => Json::Null,
        }
    }
}

impl Shl for Json {
    type Output=Json;

    fn shl(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l<<r),
            (_, _) => Json::Null,
        }
    }
}

impl BitAnd for Json {
    type Output=Json;

    fn bitand(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l&r),
            (_, _) => Json::Null,
        }
    }
}

impl BitXor for Json {
    type Output=Json;

    fn bitxor(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l^r),
            (_, _) => Json::Null,
        }
    }
}

impl BitOr for Json {
    type Output=Json;

    fn bitor(self, rhs: Json) -> Json {
        match (self.integer(), rhs.integer()) {
            (Some(l), Some(r)) => Json::new(l|r),
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

fn mixin_object(mut this: Vec<Property>, other: Vec<Property>)
    -> Vec<Property>
{
    use json::Json::{Object};

    for o in other.into_iter() {
        match property::search_by_key(&this, o.key_ref()) {
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

