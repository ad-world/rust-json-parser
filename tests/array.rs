use rust_json_parser::parsers::parse;

#[test]
fn parse_empty_array() {
    assert_eq!(parse("[]"), Ok(true));
}

#[test]
fn parse_array() {
    assert_eq!(parse("[1, 2, 3]"), Ok(true));
}

#[test]
fn parse_string_array() {
    assert_eq!(parse(r#"["hello", "world"]"#), Ok(true));
}

#[test]
fn parse_nested_array() {
    assert_eq!(parse("[[1, 2], [3, 4]]"), Ok(true));
}


#[test]
fn parse_primitive_array() {
    assert_eq!(parse("[true, false, null]"), Ok(true));
}

#[test] 
fn parse_mixed_array() {
    assert_eq!(parse(r#"[1, "hello", true, null, {}]"#), Ok(true));
}

#[test]
fn parse_object_array() {
    assert_eq!(parse(r#"[{"key": "value"}, {"key2": 123}]"#), Ok(true));
}