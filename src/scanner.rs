use std::{cell::RefCell, char, rc::Rc, str::Chars};

use crate::pos::WithPos;

pub enum MatchType<'a> {
    /// All matched
    All(&'a str, Rc<RefCell<bool>>),
    /// Match few of all or does not match
    Few(&'a str, Rc<RefCell<bool>>),
}

impl<'a> MatchType<'a> {
    /// return matched part
    pub fn value(&self) -> &'a str {
        match self {
            Self::All(v, _) => v,
            Self::Few(v, _) => v,
        }
    }
    pub fn is_matched(&self) -> bool {
        match self {
            Self::All(_, _) => true,
            Self::Few(_, _) => false,
        }
    }
    pub fn is_not_matched(&self) -> bool {
        match self {
            Self::All(_, _) => false,
            Self::Few(_, _) => true,
        }
    }
    /// on matched, consume match part
    pub fn consume_on_match(&self, v: bool) {
        match self {
            Self::All(_, is_consume) => {
                *is_consume.borrow_mut() = v;
            }
            _ => {}
        }
    }
    /// on few matched, consume match part
    pub fn consume_on_not_match(&self, v: bool) {
        match self {
            Self::Few(_, is_consume) => {
                *is_consume.borrow_mut() = v;
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
struct Matcher<'a> {
    chars: Rc<RefCell<Chars<'a>>>,
    byte_pos: Rc<RefCell<usize>>,
    line: Rc<RefCell<usize>>,
    match_next: Rc<RefCell<bool>>,
}

#[derive(Clone)]
pub struct Scanny<'a> {
    whole: &'a str,
    chars: Rc<RefCell<Chars<'a>>>,
    byte_pos: Rc<RefCell<usize>>,
    line: Rc<RefCell<usize>>,
    matcher: Rc<RefCell<Option<Matcher<'a>>>>,
}

impl<'a> From<&'a str> for Scanny<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self {
            whole: value,
            chars: Rc::new(RefCell::new(value.chars())),
            byte_pos: Rc::new(RefCell::new(0)),
            line: Rc::new(RefCell::new(1)),
            matcher: Rc::new(RefCell::new(None)),
        }
    }
}

impl<'a> Scanny<'a> {
    #[inline]
    pub fn new(value: &'a str) -> Self {
        Self::from(value)
    }
    fn next_match(&self) -> bool {
        let m = self.matcher.borrow().clone();
        if let Some(matcher) = m {
            *matcher.match_next.borrow()
        } else {
            true
        }
    }
    fn set_next_match(&self, v: bool) {
        let m = self.matcher.clone();
        let mut m = m.borrow_mut();
        if m.is_none() {
            return;
        }
        *m.as_mut().unwrap().match_next.borrow_mut() = v;
    }
    pub fn matcher(&self) -> &Self {
        if self.matcher.borrow().is_some() {
            return self;
        }
        let chars = (*self.chars.borrow()).clone();
        let byte_pos = (*self.byte_pos.borrow()).clone();
        let line = (*self.line.borrow()).clone();
        let matcher = Matcher {
            chars: Rc::new(RefCell::new(chars)),
            byte_pos: Rc::new(RefCell::new(byte_pos)),
            line: Rc::new(RefCell::new(line)),
            match_next: Rc::new(RefCell::new(true)),
        };
        *self.matcher.borrow_mut() = Some(matcher);
        self
    }
    pub fn peek(&self) -> Option<char> {
        let mut chars = if self.matcher.borrow().is_some() {
            (*self.matcher.borrow().as_ref().unwrap().chars.borrow()).clone()
        } else {
            (*self.chars.borrow()).clone()
        };
        chars.next()
    }
    pub fn peek_second(&self) -> Option<char> {
        let mut chars = if self.matcher.borrow().is_some() {
            (*self.matcher.borrow().as_ref().unwrap().chars.borrow()).clone()
        } else {
            (*self.chars.borrow()).clone()
        };
        chars.next();
        chars.next()
    }
    pub fn peek_third(&self) -> Option<char> {
        let mut chars = if self.matcher.borrow().is_some() {
            (*self.matcher.borrow().as_ref().unwrap().chars.borrow()).clone()
        } else {
            (*self.chars.borrow()).clone()
        };
        chars.next();
        chars.next();
        chars.next()
    }
    pub fn peek_nth(&self, n: usize) -> Option<char> {
        let mut chars = if self.matcher.borrow().is_some() {
            (*self.matcher.borrow().as_ref().unwrap().chars.borrow()).clone()
        } else {
            (*self.chars.borrow()).clone()
        };
        chars.nth(n)
    }
    pub fn bump(&self) -> Option<char> {
        if self.matcher.borrow().is_some() {
            let matcher = self.matcher.clone().borrow_mut().clone().unwrap();
            match matcher.chars.borrow_mut().next() {
                v @ Some('\n') => {
                    *matcher.byte_pos.borrow_mut() += 1;
                    *matcher.line.borrow_mut() += 1;
                    v
                }
                v @ Some(ch) => {
                    *matcher.byte_pos.borrow_mut() += ch.len_utf8();
                    v
                }
                v @ None => v,
            }
        } else {
            match self.chars.borrow_mut().next() {
                v @ Some('\n') => {
                    *self.byte_pos.borrow_mut() += 1;
                    *self.line.borrow_mut() += 1;
                    v
                }
                v @ Some(ch) => {
                    *self.byte_pos.borrow_mut() += ch.len_utf8();
                    v
                }
                v @ None => v,
            }
        }
    }
    pub fn skeep_while<F: Fn(char) -> bool>(&self, f: F) -> &Self {
        while self.peek().map_or(false, &f) {
            self.bump();
        }
        self
    }
    pub fn match_char<F: Fn(&char) -> bool>(&self, f: F) -> &Self {
        match self.peek() {
            Some(ch) => {
                if f(&ch) {
                    self.bump();
                    self
                } else {
                    self.set_next_match(false);
                    self
                }
            }
            None => self,
        }
    }
    pub fn match_char_optional<F: Fn(&char) -> bool>(&self, f: F) -> &Self {
        match self.peek() {
            Some(ch) => {
                if f(&ch) {
                    self.bump();
                    self
                } else {
                    self
                }
            }
            None => self,
        }
    }
    pub fn then(&self, ch: char) -> &Self {
        match self.peek() {
            Some(c) if c == ch => {
                self.bump();
                self
            }
            _ => {
                self.set_next_match(false);
                self
            }
        }
    }
    pub fn then_optional(&self, ch: char) -> &Self {
        match self.peek() {
            Some(c) if c == ch => {
                self.bump();
                self
            }
            _ => self,
        }
    }
    pub fn then_any<F: Fn(Option<char>) -> bool>(&self, f: F) -> &Self {
        if f(self.peek()) {
            self.bump();
            self
        } else {
            self.set_next_match(false);
            self
        }
    }
    pub fn then_peek<F: Fn(Self) -> bool>(&self, f: F) -> &Self {
        if f(self.clone()) {
            self
        } else {
            self.set_next_match(false);
            self
        }
    }
    pub fn then_any_optional(&self, chars: &[char]) -> &Self {
        match self.peek() {
            Some(c) if chars.contains(&c) => {
                self.bump();
                self
            }
            _ => self,
        }
    }
    pub fn peek_and_consume<F: Fn(Self) -> bool>(&self, f: F) -> &Self {
        loop {
            if f(self.clone()) {
                self.bump();
            } else {
                break;
            }
        }
        self
    }
    pub fn consume_while<F: Fn(&char) -> bool>(&self, f: F) -> &Self {
        loop {
            match self.peek() {
                Some(ch) if f(&ch) => {
                    self.bump();
                }
                _ => break,
            }
        }
        self
    }
    pub fn finalize<T, F: Fn(MatchType<'a>) -> T>(&self, f: F) -> Option<WithPos<T>> {
        let matcher = self.matcher.borrow_mut().take()?;
        let byte_pos = self.byte_pos.borrow().clone()..matcher.byte_pos.borrow().clone();
        let line_pos = self.line.borrow().clone()..=matcher.line.borrow().clone();
        let matched = self.whole.get(byte_pos.clone()).unwrap();
        let consume_on_match = Rc::new(RefCell::new(true));
        let consume_on_not_match = Rc::new(RefCell::new(true));
        let got = f(if *matcher.match_next.borrow() {
            MatchType::All(matched, consume_on_match.clone())
        } else {
            MatchType::Few(matched, consume_on_not_match.clone())
        });
        if *matcher.match_next.borrow() {
            if *consume_on_match.borrow() {
                *self.chars.borrow_mut() = matcher.chars.borrow().clone();
                *self.byte_pos.borrow_mut() = *matcher.byte_pos.borrow();
                *self.line.borrow_mut() = *matcher.line.borrow();
            }
        } else {
            if *consume_on_not_match.borrow() {
                *self.chars.borrow_mut() = matcher.chars.borrow().clone();
                *self.byte_pos.borrow_mut() = *matcher.byte_pos.borrow();
                *self.line.borrow_mut() = *matcher.line.borrow();
            }
        }
        Some(
            WithPos::new(got)
                .set_byte_pos(byte_pos)
                .set_line_pos(line_pos),
        )
    }
}

#[test]
#[ignore = "reason"]
fn test_scanny() {
    let sc = Scanny::from(
        r#"
let foo_bar = "Hello \" \
World";
   1234.567
        "#,
    );
    let mut items = Vec::new();
    loop {
        sc.skeep_while(char::is_whitespace);
        let peek = sc.peek();
        if peek.is_none() {
            break;
        }
        match peek.unwrap() {
            v if v.is_ascii_alphabetic() || v == '_' => {
                let matched = sc
                    .matcher()
                    .consume_while(|v| v.is_ascii_alphanumeric() || v == &'_')
                    .finalize(|m| m.value());
                items.push(matched);
            }
            '"' => {
                let matched = sc
                    .matcher()
                    .then('"')
                    .peek_and_consume(|scanner| {
                        if scanner.peek() == Some('\\') {
                            scanner.bump();
                        }
                        scanner.peek() != Some('"')
                    })
                    .then('"')
                    .finalize(|m| m.value());
            }
            v if v.is_ascii_digit() => {
                let num = sc
                    .matcher()
                    .consume_while(char::is_ascii_digit)
                    .then('$')
                    .consume_while(char::is_ascii_digit)
                    .then_optional(';')
                    .finalize(|matched| matched.value())
                    .unwrap();
                // assert_eq!(num, WithPos::from(("100", 2..7, 1..=1)));
            }
            _ => {
                sc.bump();
            }
        }
    }
    println!("{:?}", items);
}

#[cfg(test)]
mod tests {
    use super::Scanny;

    #[test]
    fn test_bump() {
        let sc = Scanny::new("abcd");
        assert_eq!(sc.bump(), Some('a'));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("abcd");
        sc.matcher();
        assert_eq!(sc.bump(), Some('a'));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);
        sc.finalize(|_| {});
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("abcd");
        sc.matcher();
        assert_eq!(sc.bump(), Some('a'));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);
        sc.finalize(|m| {
            m.consume_on_match(false);
        });
        assert_eq!(sc.bump(), Some('a'));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("abcd");
        sc.bump(); // consume 'a'
        sc.bump(); // consume 'b'
        sc.matcher();
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);
        sc.finalize(|m| {
            m.consume_on_match(false);
        });
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.bump(), None);
    }
    #[test]
    fn test_peek() {
        let sc = Scanny::new("abcd");
        assert_eq!(sc.peek(), Some('a'));
        assert_eq!(sc.peek(), Some('a'));
        assert_eq!(sc.bump(), Some('a'));
        assert_eq!(sc.peek(), Some('b'));
        assert_eq!(sc.peek(), Some('b'));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.peek(), Some('c'));
        assert_eq!(sc.peek(), Some('c'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.peek(), Some('d'));
        assert_eq!(sc.peek(), Some('d'));
        assert_eq!(sc.bump(), Some('d'));
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("abcd");
        sc.bump();
        sc.matcher();
        assert_eq!(sc.peek(), Some('b'));
        assert_eq!(sc.bump(), Some('b'));
        sc.finalize(|_| {});
        assert_eq!(sc.peek(), Some('c'));
        assert_eq!(sc.bump(), Some('c'));
        sc.matcher();
        assert_eq!(sc.peek(), Some('d'));
        assert_eq!(sc.bump(), Some('d'));
        sc.finalize(|_| {});
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.bump(), None);
        sc.matcher();
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.bump(), None);
        sc.finalize(|_| {});
        assert_eq!(sc.peek(), None);
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("abcd");
        assert_eq!(sc.peek(), Some('a'));
        assert_eq!(sc.peek_second(), Some('b'));
        assert_eq!(sc.peek_third(), Some('c'));
        assert_eq!(sc.peek_nth(3), Some('d'));
        assert_eq!(sc.peek_nth(4), None);
        assert_eq!(sc.bump(), Some('a'));

        let sc = Scanny::new("abcd");
        sc.bump(); // a
        sc.matcher();
        sc.bump(); // b
        sc.bump(); // c
        sc.finalize(|m| m.consume_on_match(false));
        assert_eq!(sc.bump(), Some('b'));
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), Some('d'));
    }
    #[test]
    fn test_skeep_while() {
        let sc = Scanny::new("    1234abc   ");
        sc.skeep_while(char::is_whitespace);
        assert_eq!(sc.bump(), Some('1'));
        sc.skeep_while(|v| v.is_ascii_digit());
        assert_eq!(sc.bump(), Some('a'));

        let sc = Scanny::new("    1234abc   ");
        sc.matcher();
        sc.skeep_while(char::is_whitespace);
        assert_eq!(sc.bump(), Some('1'));
        sc.skeep_while(|v| v.is_ascii_digit());
        assert_eq!(sc.bump(), Some('a'));
        sc.finalize(|m| {
            m.consume_on_match(false);
            m.consume_on_not_match(false);
        });
        sc.skeep_while(char::is_whitespace);
        assert_eq!(sc.bump(), Some('1'));
    }
    #[test]
    fn test_match_char() {
        let sc = Scanny::new("1234abc");
        sc.match_char(|v| *v == '1');
        assert_eq!(sc.bump(), Some('2'));

        let sc = Scanny::new("1234abc");
        sc.match_char(|v| *v == '2');
        assert_eq!(sc.bump(), Some('1'));

        let sc = Scanny::new("1234abc");
        sc.matcher()
            .match_char(|v| *v == '1')
            .match_char(|v| *v == '2');
        sc.bump();
        sc.bump();
        sc.match_char(|v| *v == 'a').match_char(|v| *v == 'b');
        let matched = sc.finalize(|m| {
            assert!(m.is_matched());
            assert!(!m.is_not_matched());
            m.value()
        });
        assert_eq!(matched.unwrap().value, "1234ab");
        assert_eq!(sc.bump(), Some('c'));
        assert_eq!(sc.bump(), None);

        let sc = Scanny::new("1234abc");
        sc.matcher()
            .match_char(|v| *v == '1')
            .match_char(|v| *v == '2');
        sc.bump();
        sc.bump();
        sc.match_char(|v| *v == 'b').match_char(|v| *v == 'b');
        let matched = sc.finalize(|m| {
            assert!(!m.is_matched());
            assert!(m.is_not_matched());
            m.value()
        });
        assert_eq!(matched.unwrap().value, "1234");
        assert_eq!(sc.bump(), Some('a'));
    }
    #[test]
    fn test_peek_and_consume() {
        let sc = Scanny::new(r"    'ab\' cd''hello world'   ");
        sc.skeep_while(char::is_whitespace);
        let matched = sc
            .matcher()
            .then('\'')
            .peek_and_consume(|v| match v.peek() {
                Some('\\') => {
                    v.bump();
                    true
                }
                Some('\'') => false,
                _ => true,
            })
            .then('\'')
            .finalize(|v| v.value())
            .unwrap()
            .value;
        assert_eq!(matched, r"'ab\' cd'");
        assert_eq!(sc.bump(), Some('\''));
        assert_eq!(sc.bump(), Some('h'));
    }
}
