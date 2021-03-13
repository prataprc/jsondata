// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

// TODO: replace [u8; 32] to [u8; 64] once constant generic is available
// in rust.

use std::cmp::Ordering;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub enum Integral {
    Text { len: usize, bytes: [u8; 32] },
    Data { value: i128 },
}

impl Integral {
    pub fn new<T>(val: T) -> Integral
    where
        Self: From<T>,
    {
        val.into()
    }

    pub fn integer(&self) -> Option<i128> {
        use std::str::from_utf8;
        match self {
            Integral::Data { value } => Some(*value),
            Integral::Text { len, bytes } => {
                let s = &bytes[0..*len];
                if s.len() > 2 && s[0] == 48 && s[1] == 120
                // "0x"
                {
                    i128::from_str_radix(from_utf8(&s[2..]).unwrap(), 16).ok()
                } else if s.len() > 3 && s[0] == 45 && s[1] == 48 && s[2] == 120
                // "-0x"
                {
                    i128::from_str_radix(from_utf8(&s[3..]).unwrap(), 16)
                        .map(|x| -x)
                        .ok()
                } else {
                    from_utf8(s).unwrap().parse::<i128>().ok()
                }
            }
        }
    }

    pub fn compute(&mut self) -> Result<()> {
        use std::str::from_utf8;

        match self {
            Integral::Data { .. } => Ok(()),
            Integral::Text { len, bytes } => {
                let s = &bytes[0..*len];
                let res = if s.len() > 2 && s[0] == 48 && s[1] == 120
                // "0x"
                {
                    i128::from_str_radix(from_utf8(&s[2..]).unwrap(), 16)
                } else if s.len() > 3 && s[0] == 45 && s[1] == 48 && s[2] == 120
                // "-0x"
                {
                    let s = from_utf8(&s[3..]).unwrap();
                    i128::from_str_radix(s, 16).map(|x| -x)
                } else {
                    from_utf8(s).unwrap().parse::<i128>()
                };
                match res {
                    Ok(value) => {
                        *self = Integral::Data { value };
                        Ok(())
                    }
                    Err(err) => Err(Error::InvalidNumber(format!("{}", err))),
                }
            }
        }
    }
}

impl From<i128> for Integral {
    fn from(value: i128) -> Integral {
        Integral::Data { value }
    }
}

impl<'a> From<&'a str> for Integral {
    fn from(val: &str) -> Integral {
        let src = val.as_bytes();
        let mut bytes = [0_u8; 32];
        bytes[..src.len()].copy_from_slice(src);
        Integral::Text {
            len: val.len(),
            bytes,
        }
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

#[derive(Clone, Debug)]
pub enum Floating {
    Text { len: usize, bytes: [u8; 32] },
    Data { value: f64 },
}

impl Floating {
    pub fn new<T>(val: T) -> Floating
    where
        Self: From<T>,
    {
        val.into()
    }

    pub fn float(&self) -> Option<f64> {
        use std::str::from_utf8;

        match self {
            Floating::Data { value } => Some(*value),
            Floating::Text { len, bytes } => {
                from_utf8(&bytes[0..*len]).unwrap().parse::<f64>().ok()
            }
        }
    }

    pub fn compute(&mut self) -> Result<()> {
        use std::str::from_utf8;

        match self {
            Floating::Data { .. } => Ok(()),
            Floating::Text { len, bytes } => {
                match from_utf8(&bytes[0..*len]).unwrap().parse::<f64>() {
                    Ok(value) => {
                        *self = Floating::Data { value };
                        Ok(())
                    }
                    Err(err) => Err(Error::InvalidNumber(format!("{}", err))),
                }
            }
        }
    }
}

impl From<f64> for Floating {
    fn from(value: f64) -> Floating {
        Floating::Data { value }
    }
}

impl<'a> From<&'a str> for Floating {
    fn from(val: &str) -> Floating {
        let src = val.as_bytes();
        let mut bytes = [0_u8; 32];
        bytes[..src.len()].copy_from_slice(src);
        Floating::Text {
            len: val.len(),
            bytes,
        }
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
