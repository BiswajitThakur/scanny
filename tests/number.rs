use std::ops::{Range, RangeInclusive};

use scanny::{MatchType, Scanny, WithPos};

fn get_float<'a>(sc: &'a Scanny<'a>) -> WithPos<MatchType<'a>> {
    sc.skeep_while(|v| !v.is_ascii_digit())
        .matcher()
        .then_any(|v| {
            if let Some(d) = v {
                d.is_ascii_digit()
            } else {
                false
            }
        })
        .consume_while(|v| v.is_ascii_digit() || *v == '_')
        .then('.')
        .then_peek(|v| match v.peek() {
            Some(ch) if ch.is_ascii_digit() => {
                v.bump();
                true
            }
            Some(ch) if ch.is_whitespace() => {
                v.matched();
                true
            }
            Some(';') => {
                v.matched();
                true
            }
            Some(_) => false,
            None => true,
        })
        .consume_while(|v| v.is_ascii_digit() || *v == '_')
        .then_optional('f')
        .then_peek(|v| match v.peek() {
            Some(ch) if ch.is_whitespace() => true,
            None => true,
            _ => false,
        })
        .finalize(|v| v)
        .unwrap()
}

macro_rules! tf {
    ($sc:ident, $value:expr, $matched:expr) => {
        let first_match = get_float(&$sc).value;
        if $matched {
            assert!(first_match.is_matched());
            assert!(!first_match.is_not_matched());
        } else {
            assert!(!first_match.is_matched());
            assert!(first_match.is_not_matched());
        }
        assert_eq!(first_match.value(), $value);
    };
}

#[test]
fn test_int() {
    let sc = Scanny::new("  abc  1234567n");
    let matched = sc
        .skeep_while(|v| !v.is_ascii_digit())
        .matcher()
        .consume_while(|v| v.is_ascii_digit())
        .finalize(|v| v.value())
        .unwrap()
        .value;
    assert_eq!(matched, "1234567");

    let sc = Scanny::new("  abc  1234567n");
    let matched = sc
        .skeep_while(|v| !v.is_ascii_digit())
        .matcher()
        .consume_while(|v| v.is_ascii_digit())
        .then_optional('n')
        .finalize(|v| v.value())
        .unwrap()
        .value;
    assert_eq!(matched, "1234567n");
}

#[test]
fn test_match_float_1() {
    let sc = Scanny::new("  abc 123 def 321.56fh 765.32f xyz 33.44 bjh 55.3f");
    tf!(sc, "123", false);
    tf!(sc, "321.56f", false);
    tf!(sc, "765.32f", true);
    tf!(sc, "33.44", true);
    tf!(sc, "55.3f", true);
}

fn test_float<'a>(
    sc: &'a Scanny<'a>,
    tests: Vec<(RangeInclusive<usize>, Range<usize>, &str, bool)>,
) {
    for test in tests {
        let match_result = get_float(sc);
        assert_eq!(match_result.get_line_pos(), test.0);
        assert_eq!(match_result.get_byte_pos(), test.1);
        assert_eq!(match_result.value.value(), test.2);
        assert_eq!(match_result.value.is_matched(), test.3);
        assert_eq!(match_result.value.is_not_matched(), !test.3);
    }
}

#[test]
fn test_match_float_2() {
    let sc = Scanny::new(
        r#"hello 123 456 67.56ff 1_2.3_2 world 44_.55f
34. 56 .45 90___.45 foo 999. bar 45_67f 56..32
33._33f 44.44_f a123.777 hello _56.76 555.;
        "#,
    );
    test_float(
        &sc,
        vec![
            (1..=1, 6..9, "123", false),
            (1..=1, 10..13, "456", false),
            (1..=1, 14..20, "67.56f", false),
            (1..=1, 22..29, "1_2.3_2", true),
            (1..=1, 36..43, "44_.55f", true),
            (2..=2, 44..47, "34.", true),
            (2..=2, 48..50, "56", false),
            (2..=2, 52..54, "45", false),
            (2..=2, 55..63, "90___.45", true),
            (2..=2, 68..72, "999.", true),
            (2..=2, 77..82, "45_67", false),
            (2..=2, 84..87, "56.", false),
            (2..=2, 88..90, "32", false),
            (3..=3, 91..94, "33.", false),
            (3..=3, 95..97, "33", false),
            (3..=3, 99..106, "44.44_f", true),
            (3..=3, 108..115, "123.777", true),
            (3..=3, 123..128, "56.76", true),
            (3..=3, 129..133, "555.", true),
        ],
    );
}
