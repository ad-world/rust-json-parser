use rust_json_parser::parsers::parse;

#[test]
fn parse_kv_pair() {
    assert_eq!(parse(r#"{"key": "value"}"#), Ok(true));
}

#[test]
fn parse_different_values() {
    assert_eq!(
        parse(r#"
        {
            "key": "value",
            "key2": 123,
            "key3": 123.456,
            "key4": true,
            "key5": false,
            "key6": null
        }
        "#),
        Ok(true)
    )
}
