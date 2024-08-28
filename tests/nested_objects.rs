use rust_json_parser::parsers::parse;

#[test]
fn parse_object_with_object() {
    assert_eq!(
        parse(r#"
        {
            "key": {
                "key2": "value"
            }
        }
        "#),
        Ok(true)
    )
}

#[test]
fn parse_object_with_array() {
    assert_eq!(
        parse(r#"
        {
            "key": [1, 2, 3]
        }
        "#),
        Ok(true)
    )
}

#[test]
fn parse_object_with_object_array() {
    assert_eq!(
        parse(r#"
        {
            "key": [{"key": "value"}, {"key2": 123}]
        }
        "#),
        Ok(true)
    )
}