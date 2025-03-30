#![allow(unused)]
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc, str::Chars, usize};

use crate::pos::WithPos;

struct TokenMatcher<'a> {
    start: Option<(usize, usize)>,
    tokenizer: Scanner<'a>,
    is_prev_match: bool,
}

impl<'a> TokenMatcher<'a> {
    fn new(tokenizer: Scanner<'a>) -> Self {
        Self {
            start: None,
            tokenizer,
            is_prev_match: true,
        }
    }
    pub fn and_then(&mut self, ch: char, matched: T) -> &mut Self {
        if !self.is_prev_match {
            return self;
        }
        match self.tokenizer.peek() {
            Some(c) if c == ch => {
                if self.start.is_none() {
                    self.start = Some(self.tokenizer.pos());
                }
                self.tokenizer.bump();
                self.value = Some(matched);
                self
            }
            _ => {
                self.is_prev_match = false;
                self
            }
        }
    }
    pub fn then<F: Fn(char) -> Option<T>>(&mut self, f: F) -> &mut Self {
        todo!()
    }
    pub fn finalized(&self) -> Option<WithPos<&str>> {
        let start = self.start?;
        let end_pos = self.tokenizer.pos();
        let line_pos = start.0..end_pos.0;
        let byte_pos = start.1..end_pos.1;
        let value = self.value.clone()?;
        Some(
            WithPos::new(value)
                .set_byte_pos(byte_pos)
                .set_line_pos(line_pos),
        )
    }
}

pub struct Scanner<'a> {
    whole: Rc<&'a str>,
    input: Rc<RefCell<Chars<'a>>>,
    line: Rc<RefCell<usize>>,
    pos: Rc<RefCell<usize>>,
}

impl Clone for Scanner<'_> {
    fn clone(&self) -> Self {
        Self {
            whole: self.whole.clone(),
            input: self.input.clone(),
            line: self.line.clone(),
            pos: self.pos.clone(),
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            whole: Rc::new(value),
            input: Rc::new(RefCell::new(value.chars())),
            line: Rc::new(RefCell::new(0)),
            pos: Rc::new(RefCell::new(0)),
        }
    }
    #[inline]
    pub(crate) fn bump(&mut self) -> Option<char> {
        match self.input.borrow_mut().next() {
            ch @ Some('\n') => {
                *self.pos.borrow_mut() += 1; // utf-8 length of new line is 1
                *self.line.borrow_mut() += 1;
                ch
            }
            ch @ Some(c) => {
                *self.pos.borrow_mut() += c.len_utf8();
                ch
            }
            none => none,
        }
    }
    #[inline]
    pub(crate) fn peek(&self) -> Option<char> {
        self.input.borrow().clone().next()
    }
    pub(crate) fn pos(&self) -> (usize, usize) {
        (self.line.borrow().clone(), self.pos.borrow().clone())
    }
    pub(crate) fn consume_while<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let mut v = String::new();
        loop {
            match self.peek() {
                Some(ch) if f(ch) => {
                    self.bump();
                    v.push(ch);
                }
                _ => break,
            }
        }
        v
    }
    pub(crate) fn token_matcher(&mut self) -> TokenMatcher<'a, &'a str> {
        TokenMatcher::new(self.clone())
    }
}

/*

token_matcher()
    .then(|v|todo())
    .and_then()

*/
