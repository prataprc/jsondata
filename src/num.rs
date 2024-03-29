// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use std::cmp::Ordering;

use crate::{Error, Result};

#[inline]
fn parse_integer(text: &[u8]) -> Result<i128> {
    use std::str::from_utf8_unchecked;

    let res = unsafe {
        if text.len() > 2 && text[0] == 48 && text[1] == 120
        // "0x"
        {
            i128::from_str_radix(from_utf8_unchecked(&text[2..]), 16)
        } else if text.len() > 3 && text[0] == 45 && text[1] == 48 && text[2] == 120
        // "-0x"
        {
            i128::from_str_radix(from_utf8_unchecked(&text[3..]), 16).map(|x| -x)
        } else {
            from_utf8_unchecked(text).parse::<i128>()
        }
    };
    err_at!(InvalidNumber, res)
}

#[inline]
fn parse_float(text: &[u8]) -> Result<f64> {
    use std::str::from_utf8_unchecked;

    err_at!(InvalidNumber, unsafe { from_utf8_unchecked(text).parse::<f64>() })
}

#[derive(Clone, Debug)]
pub enum Integral {
    Text { len: usize, bytes: [u8; 128] },
    Data { value: i128 },
}

macro_rules! convert_to_integral {
    ($($from:ty),*) => (
        $(
            impl From<$from> for Integral {
                fn from(val: $from) -> Integral {
                    Integral::Data { value: i128::try_from(val).unwrap() }
                }
            }
        )*
    );
}

convert_to_integral! {u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize }

impl<'a> TryFrom<&'a str> for Integral {
    type Error = Error;

    fn try_from(val: &str) -> Result<Integral> {
        let src = val.as_bytes();
        let val = match src.len() {
            n if n < 128 => {
                let mut bytes = [0_u8; 128];
                bytes[..n].copy_from_slice(src);
                Integral::Text { len: n, bytes }
            }
            _ => {
                let value = parse_integer(src)?;
                Integral::Data { value }
            }
        };

        Ok(val)
    }
}

impl Eq for Integral {}

impl PartialEq for Integral {
    fn eq(&self, other: &Integral) -> bool {
        use Integral::{Data, Text};

        match (self, other) {
            (Data { value: a }, Data { value: b }) => a.eq(b),
            (Text { len, bytes }, Data { value: b }) => {
                parse_integer(&bytes[..*len]).map(|a| a.eq(b)).unwrap()
            }
            (Data { value: a }, Text { len, bytes }) => {
                parse_integer(&bytes[..*len]).map(|b| a.eq(&b)).unwrap()
            }
            (
                Text { len: a_len, bytes: a_bytes },
                Text { len: b_len, bytes: b_bytes },
            ) => {
                let a = parse_integer(&a_bytes[..*a_len]).unwrap();
                let b = parse_integer(&b_bytes[..*b_len]).unwrap();
                a.eq(&b)
            }
        }
    }
}

impl PartialOrd for Integral {
    fn partial_cmp(&self, other: &Integral) -> Option<Ordering> {
        use Integral::{Data, Text};

        match (self, other) {
            (Data { value: a }, Data { value: b }) => a.partial_cmp(b),
            (Text { len, bytes }, Data { value: b }) => {
                match parse_integer(&bytes[..*len]) {
                    Ok(a) => a.partial_cmp(b),
                    _ => None,
                }
            }
            (Data { value: a }, Text { len, bytes }) => {
                match parse_integer(&bytes[..*len]) {
                    Ok(b) => a.partial_cmp(&b),
                    _ => None,
                }
            }
            (
                Text { len: a_len, bytes: a_bytes },
                Text { len: b_len, bytes: b_bytes },
            ) => {
                let a = parse_integer(&a_bytes[..*a_len]).ok()?;
                let b = parse_integer(&b_bytes[..*b_len]).ok()?;
                a.partial_cmp(&b)
            }
        }
    }
}

impl Integral {
    pub fn integer(&self) -> Option<i128> {
        match self {
            Integral::Data { value } => Some(*value),
            Integral::Text { len, bytes } => parse_integer(&bytes[0..*len]).ok(),
        }
    }

    pub fn integer_result(&self) -> Result<i128> {
        match self {
            Integral::Data { value } => Ok(*value),
            Integral::Text { len, bytes } => parse_integer(&bytes[0..*len]),
        }
    }

    pub fn float(&self) -> Option<f64> {
        let val = self.integer()?;
        if (-9007199254740992..=9007199254740992).contains(&val) {
            Some(val as f64)
        } else {
            // TODO: strict accuracy or tolerant behaviour
            None
        }
    }

    pub fn compute(&mut self) -> Result<()> {
        if let Integral::Text { len, bytes } = self {
            let value = parse_integer(&bytes[0..*len])?;
            *self = Integral::Data { value };
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Floating {
    Text { len: usize, bytes: [u8; 128] },
    Data { value: f64 },
}

macro_rules! convert_to_float {
    ($($from:ty),*) => (
        $(
            impl From<$from> for Floating {
                fn from(val: $from) -> Floating {
                    Floating::Data { value: val.into() }
                }
            }
        )*
    );
}

convert_to_float! {f32, f64}

impl<'a> TryFrom<&'a str> for Floating {
    type Error = Error;

    fn try_from(val: &str) -> Result<Floating> {
        let src = val.as_bytes();
        let val = match src.len() {
            n if n < 128 => {
                let mut bytes = [0_u8; 128];
                bytes[..src.len()].copy_from_slice(src);
                Floating::Text { len: val.len(), bytes }
            }
            _ => {
                let value = parse_float(src)?;
                Floating::Data { value }
            }
        };

        Ok(val)
    }
}

impl Eq for Floating {}

impl PartialEq for Floating {
    fn eq(&self, other: &Floating) -> bool {
        use Floating::{Data, Text};

        match (self, other) {
            (Data { value: a }, Data { value: b }) => a.eq(b),
            (Text { len, bytes }, Data { value: b }) => {
                parse_float(&bytes[..*len]).map(|a| a.eq(b)).unwrap()
            }
            (Data { value: a }, Text { len, bytes }) => {
                parse_float(&bytes[..*len]).map(|b| a.eq(&b)).unwrap()
            }
            (
                Text { len: a_len, bytes: a_bytes },
                Text { len: b_len, bytes: b_bytes },
            ) => {
                let a = parse_float(&a_bytes[..*a_len]).unwrap();
                let b = parse_float(&b_bytes[..*b_len]).unwrap();
                a.eq(&b)
            }
        }
    }
}

impl PartialOrd for Floating {
    fn partial_cmp(&self, other: &Floating) -> Option<Ordering> {
        use Floating::{Data, Text};

        match (self, other) {
            (Data { value: a }, Data { value: b }) => Some(a.total_cmp(b)),
            (Text { len, bytes }, Data { value: b }) => {
                parse_float(&bytes[..*len]).map(|a| a.total_cmp(b)).ok()
            }
            (Data { value: a }, Text { len, bytes }) => {
                parse_float(&bytes[..*len]).map(|b| a.total_cmp(&b)).ok()
            }
            (
                Text { len: a_len, bytes: a_bytes },
                Text { len: b_len, bytes: b_bytes },
            ) => {
                let a = parse_float(&a_bytes[..*a_len]).unwrap();
                let b = parse_float(&b_bytes[..*b_len]).unwrap();
                Some(a.total_cmp(&b))
            }
        }
    }
}

impl Floating {
    pub fn float(&self) -> Option<f64> {
        match self {
            Floating::Data { value } => Some(*value),
            Floating::Text { len, bytes } => parse_float(&bytes[0..*len]).ok(),
        }
    }

    pub fn float_result(&self) -> Result<f64> {
        match self {
            Floating::Data { value } => Ok(*value),
            Floating::Text { len, bytes } => parse_float(&bytes[0..*len]),
        }
    }

    pub fn compute(&mut self) -> Result<()> {
        if let Floating::Text { len, bytes } = self {
            let value = parse_float(&bytes[..*len])?;
            *self = Floating::Data { value };
        }

        Ok(())
    }
}
