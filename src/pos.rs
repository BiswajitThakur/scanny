use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct WithPos<T> {
    value: T,
    byte_pos: Range<usize>,
    line_pos: Range<usize>,
}

impl<T> WithPos<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            byte_pos: 0..0,
            line_pos: 0..0,
        }
    }
    pub fn set_byte_pos(mut self, pos: Range<usize>) -> Self {
        self.byte_pos = pos;
        self
    }
    pub fn set_line_pos(mut self, pos: Range<usize>) -> Self {
        self.line_pos = pos;
        self
    }
}
