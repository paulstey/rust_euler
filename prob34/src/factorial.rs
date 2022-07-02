//! /src/iterator.rs

#[derive(Debug, Eq, PartialEq)]
pub struct Factorial {
    pair: (i64, i64),
    end: i64,
}

impl Factorial {
    pub fn new(start: i64) -> Self {
        Factorial {
            pair: (1, 1),
            end: start,
        }
    }
}

impl Iterator for Factorial {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pair.0 < self.end {
            self.pair.0 += 1;
            self.pair.1 *= self.pair.0;
            Some(self.pair)
        } else {
            None
        }
    }
}
