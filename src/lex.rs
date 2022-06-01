// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

// This is local type, useful to pinpoint position of lex-failures.
#[derive(Debug)]
pub struct Lex {
    pub off: usize,
    pub row: usize,
    pub col: usize,
}

impl Lex {
    pub fn new(off: usize, row: usize, col: usize) -> Lex {
        Lex { off, row, col }
    }

    pub fn incr_col(&mut self, i: usize) {
        self.off += i;
        self.col += i;
    }

    pub fn format(&self, prefix: &str) -> String {
        format!("{} at offset:{} line:{} col:{}", prefix, self.off, self.row, self.col)
    }
}
