use std::ops::{Range, RangeInclusive};

#[derive(Debug, PartialEq)]
pub struct WithPos<T> {
    pub value: T,
    byte_pos: Range<usize>,
    line_pos: RangeInclusive<usize>,
}

impl<T> From<(T, Range<usize>, RangeInclusive<usize>)> for WithPos<T> {
    fn from(value: (T, Range<usize>, RangeInclusive<usize>)) -> Self {
        Self {
            value: value.0,
            byte_pos: value.1,
            line_pos: value.2,
        }
    }
}

impl<T> From<(T, RangeInclusive<usize>, Range<usize>)> for WithPos<T> {
    fn from(value: (T, RangeInclusive<usize>, Range<usize>)) -> Self {
        Self {
            value: value.0,
            byte_pos: value.2,
            line_pos: value.1,
        }
    }
}

impl<T> WithPos<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            byte_pos: 0..0,
            line_pos: 0..=0,
        }
    }
    pub fn set_byte_pos(mut self, pos: Range<usize>) -> Self {
        self.byte_pos = pos;
        self
    }
    pub fn set_line_pos(mut self, pos: RangeInclusive<usize>) -> Self {
        self.line_pos = pos;
        self
    }
}
