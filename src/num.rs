use std::cmp::Ordering;

#[derive(Clone,Debug)]
pub struct Integral {
    pub len: usize,
    pub txt: [u8; 32],
    pub val: Option<i128>,
}

impl Integral {
    pub fn new<T>(val: T) -> Integral where Self: From<T> {
        val.into()
    }

    pub fn integer(&self) -> Option<i128> {
        use std::str::from_utf8;
        if self.val.is_none() {
            let bs = &self.txt[0..self.len];
            if bs.len() > 2 && bs[0] == 48 /*'0'*/ && bs[1] == 120 /*'x'*/ {
                i128::from_str_radix(from_utf8(&bs[2..]).unwrap(), 16).ok()
            } else if bs.len() > 3 && bs[0] == 45 /*'-'*/ && bs[1] == 48 /*'0'*/ && bs[2] == 120 /*'x'*/ {
                i128::from_str_radix(from_utf8(&bs[3..]).unwrap(), 16).map(|x| -x).ok()
            } else {
                i128::from_str_radix(from_utf8(bs).unwrap(), 10).ok()
            }
        } else {
            self.val
        }
    }

    pub fn compute(&mut self) -> Result<(), String> {
        use std::str::from_utf8;

        //println!("{:?}", self.txt);
        if self.val.is_none() {
            let bs = &self.txt[0..self.len];
            let res = if bs.len() > 2 && bs[0] == 48 /*'0'*/ && bs[1] == 120 /*'x'*/ {
                i128::from_str_radix(from_utf8(&bs[2..]).unwrap(), 16)
            } else if bs.len() > 3 && bs[0] == 45 /*'-'*/ && bs[1] == 48 /*'0'*/ && bs[2] == 120 /*'x'*/ {
                i128::from_str_radix(from_utf8(&bs[3..]).unwrap(), 16).map(|x| -x)
            } else {
                i128::from_str_radix(from_utf8(bs).unwrap(), 10)
            };
            match res {
                Ok(val) => self.val = Some(val),
                Err(err) => return Err(format!("parse: {}", err)),
            }
        }
        Ok(())
    }
}

impl From<i128> for Integral {
    fn from(val: i128) -> Integral {
        Integral{len: 0, txt: [0_u8; 32], val: Some(val)}
    }
}

impl<'a> From<&'a str> for Integral {
    fn from(val: &str) -> Integral {
        let mut res = Integral{len: val.len(), txt: [0_u8; 32], val: None};
        res.txt[..val.len()].as_mut().copy_from_slice(val.as_bytes());
        res
    }
}


impl Eq for Integral {}

impl PartialEq for Integral {
    fn eq(&self, other: &Integral) -> bool {
        self.integer() == other.integer()
    }
}

impl PartialOrd for Integral {
    fn partial_cmp(&self, other: &Integral) -> Option<Ordering> {
        self.integer().partial_cmp(&other.integer())
    }
}


#[derive(Clone,Debug)]
pub struct Floating {
    pub len: usize,
    pub txt: [u8; 32],
    pub val: Option<f64>,
}

impl Floating {
    pub fn new<T>(val: T) -> Floating where Self: From<T> {
        val.into()
    }

    pub fn float(&self) -> Option<f64> {
        use std::str::from_utf8;

        if self.val.is_none() {
            from_utf8(&self.txt[0..self.len]).unwrap().parse::<f64>().ok()
        } else {
            self.val
        }
    }

    pub fn compute(&mut self) -> Result<(), String> {
        use std::str::from_utf8;

        if self.val.is_none() {
            match from_utf8(&self.txt[0..self.len]).unwrap().parse::<f64>() {
                Ok(val) => self.val = Some(val),
                Err(err) => return Err(format!("parse: {}", err)),
            }
        }
        Ok(())
    }
}

impl From<f64> for Floating {
    fn from(val: f64) -> Floating {
        Floating{len: 0, txt: [0_u8; 32], val: Some(val)}
    }
}

impl<'a> From<&'a str> for Floating {
    fn from(val: &str) -> Floating {
        let mut res = Floating{len: val.len(), txt: [0_u8; 32], val: None};
        res.txt[..val.len()].as_mut().copy_from_slice(val.as_bytes());
        res
    }
}

impl Eq for Floating {}

impl PartialEq for Floating {
    fn eq(&self, other: &Floating) -> bool {
        self.float() == other.float()
    }
}

impl PartialOrd for Floating {
    fn partial_cmp(&self, other: &Floating) -> Option<Ordering> {
        self.float().partial_cmp(&other.float())
    }
}



