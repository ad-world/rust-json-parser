use rust_json_parser::parsers::parse;

#[test]

fn no_closing_brace() {
    assert_ne!(parse("{"), Ok(true));
}

#[test]
fn no_closing_bracket() {
    assert_ne!(parse("["), Ok(true));
}

#[test]
fn invalid_key() {
    assert_ne!(parse("{1: 2}"), Ok(true));
}

#[test]
fn invalid_value() {
    assert_ne!(parse(r#"{"key": } }"#), Ok(true));
}

#[test]
fn invalid_literal() {
    assert_ne!(parse("nul"), Ok(true));
}

#[test]
fn invalid_number() {
    assert_ne!(parse("123.234.1"), Ok(true));
}