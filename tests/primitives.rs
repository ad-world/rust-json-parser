use rust_json_parser::parsers::parse;

#[test]
fn parse_empty_object() {
    assert_eq!(parse("{}"), Ok(true));
}

#[test]
fn parse_null() {
    assert_eq!(parse("null"), Ok(true));
}

#[test]
fn parse_true() {
    assert_eq!(parse("true"), Ok(true));
}

#[test]
fn parse_false() {
    assert_eq!(parse("false"), Ok(true));
}

#[test]
fn parse_string() {
    assert_eq!(parse("\"hello\""), Ok(true));
}

#[test]
fn parse_number() {
    assert_eq!(parse("123"), Ok(true));
}

#[test]
fn parse_array() {
    assert_eq!(parse("[1, 2, 3]"), Ok(true));
}

#[test]
fn parse_decimal_number() {
    assert_eq!(parse("123.456"), Ok(true));
}

