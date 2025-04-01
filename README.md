# Scanny

`Scanny` is a advanced text scanning library for Rust.

# Installation

```bash
cargo add scanny
```

## Example

A simple tokenizer using `scanny`.

```rust
use scanny::{Scanny, WithPos};

#[allow(dead_code)]
#[derive(Debug)]
enum Token<'a> {
    Let,
    Mut,
    Eq,
    Colon,
    Ident(&'a str),
    Number(u32),
    String(&'a str),
}

fn main() {
    let code = r#"
        let mut foo = 500;
        let bar = "Hello \" World\
'hello world' ";
        "#;
    let sc = Scanny::new(code);
    let mut tokens = Vec::new();
    loop {
        sc.skeep_while(char::is_whitespace);
        if sc.peek().is_none() {
            break;
        }
        match sc.peek().unwrap() {
            'a'..='z' | 'A'..='Z' | '_' => {
                tokens.push(consume_keywords(&sc));
            }
            '0'..='9' => tokens.push(consume_number(&sc)),
            '"' => tokens.push(consume_string(&sc)),
            '=' => {
                tokens.push(sc.matcher().then('=')
                    .finalize(|_| Token::Eq).unwrap());
            }
            ';' => {
                tokens.push(sc.matcher().then(';')
                    .finalize(|_| Token::Colon).unwrap());
            }
            t => {
                sc.bump();
                eprintln!("Invalid Token: {}", t);
            }
        }
    }
    println!("{:#?}", tokens);
}

fn consume_keywords<'a>(sc: &'a Scanny<'a>) -> WithPos<Token<'a>> {
    sc.matcher()
        .match_char(|v| matches!(v, 'a'..='z' | 'A'..='Z' | '_'))
        .consume_while(|v| v.is_ascii_alphabetic() || v.is_ascii_digit() || *v == '_')
        .finalize(|v| match v.value() {
            "let" => Token::Let,
            "mut" => Token::Mut,
            t => Token::Ident(t),
        })
        .unwrap()
}

fn consume_number<'a>(sc: &'a Scanny<'a>) -> WithPos<Token<'a>> {
    sc.matcher()
        .match_char(char::is_ascii_digit)
        .consume_while(char::is_ascii_digit)
        .finalize(|v| Token::Number(v.value().parse().unwrap()))
        .unwrap()
}

fn consume_string<'a>(sc: &'a Scanny<'a>) -> WithPos<Token<'a>> {
    sc.matcher()
        .then('"')
        .peek_and_consume(|v| match v.peek() {
            Some('\\') => {
                v.bump();
                true
            }
            Some('"') => false,
            None => false,
            _ => true,
        })
        .then('"')
        .finalize(|v| {
            let matched = v
                .value()
                .strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .unwrap();
            Token::String(matched)
        })
        .unwrap()
}
```

## Output

```txt
[
    WithPos {
        value: Let,
        byte_pos: 9..12,
        line_pos: 2..=2,
    },
    WithPos {
        value: Mut,
        byte_pos: 13..16,
        line_pos: 2..=2,
    },
    WithPos {
        value: Ident(
            "foo",
        ),
        byte_pos: 17..20,
        line_pos: 2..=2,
    },
    WithPos {
        value: Eq,
        byte_pos: 21..22,
        line_pos: 2..=2,
    },
    WithPos {
        value: Number(
            500,
        ),
        byte_pos: 23..26,
        line_pos: 2..=2,
    },
    WithPos {
        value: Colon,
        byte_pos: 26..27,
        line_pos: 2..=2,
    },
    WithPos {
        value: Let,
        byte_pos: 36..39,
        line_pos: 3..=3,
    },
    WithPos {
        value: Ident(
            "bar",
        ),
        byte_pos: 40..43,
        line_pos: 3..=3,
    },
    WithPos {
        value: Eq,
        byte_pos: 44..45,
        line_pos: 3..=3,
    },
    WithPos {
        value: String(
            "Hello \\\" World\\\n'hello world' ",
        ),
        byte_pos: 46..78,
        line_pos: 3..=4,
    },
    WithPos {
        value: Colon,
        byte_pos: 78..79,
        line_pos: 4..=4,
    },
]
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
