use scanny::{MatchType, Scanny};

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

fn get_float<'a>(sc: &'a Scanny<'a>) -> MatchType<'a> {
    sc.skeep_while(|v| !v.is_ascii_digit())
        .matcher()
        .consume_while(|v| v.is_ascii_digit())
        .then('.')
        .consume_while(|v| v.is_ascii_digit())
        .then_optional('f')
        .then_peek(|v| match v.peek() {
            Some(ch) if ch.is_whitespace() => true,
            None => true,
            _ => false,
        })
        .finalize(|v| v)
        .unwrap()
        .value
}

#[test]
fn test_float() {
    let sc = Scanny::new("  abc 123 def 321.56fh 765.32f xyz 33.44 bjh 55.3f");
    let first_match = get_float(&sc);
    assert!(!first_match.is_matched());
    assert!(first_match.is_not_matched());
    assert_eq!(first_match.value(), "123");

    let second_match = get_float(&sc);
    assert!(!second_match.is_matched());
    assert!(second_match.is_not_matched());
    assert_eq!(second_match.value(), "321.56f");

    let second_match = get_float(&sc);
    assert!(second_match.is_matched());
    assert!(!second_match.is_not_matched());
    assert_eq!(second_match.value(), "765.32f");

    let second_match = get_float(&sc);
    assert!(second_match.is_matched());
    assert!(!second_match.is_not_matched());
    assert_eq!(second_match.value(), "33.44");

    let second_match = get_float(&sc);
    assert!(second_match.is_matched());
    assert!(!second_match.is_not_matched());
    assert_eq!(second_match.value(), "55.3f");
}
