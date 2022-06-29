//! /src/iterator.rs
//!

#[derive(Debug, Eq, PartialEq)]
pub struct Factorial {
    pair: (i64, i64),
}

impl Factorial {
    pub fn new(start: i64) -> Self {
        Factorial { pair: (start, 1) }
    }
}

impl Iterator for Factorial {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pair.0 > 1 {
            self.pair.1 *= self.pair.0;
            self.pair.0 -= 1;

            Some(self.pair)
        } else {
            None
        }
    }
}
