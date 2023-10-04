mod parser;
mod tokenizer;

use crate::parser::{JsonValue, Parser};
use crate::tokenizer::Tokenizer;

pub fn parse_json_str(input: &str) -> Result<JsonValue, String> {
    let tokenizer = Tokenizer::new(input.as_bytes());
    let mut parser = Parser::new(tokenizer);
    parser.parse_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        let json_str = "";
        match parse_json_str(json_str) {
            Err(e) => assert_eq!(e, "Unexpected token"),
            _ => panic!("Expected error when parsing empty string"),
        }
    }

    #[test]
    fn parse_empty_object() {
        let json_str = "{}";
        match parse_json_str(json_str) {
            Ok(JsonValue::Object { kv }) => assert_eq!(kv.len(), 0),
            _ => panic!("Expected empty object"),
        }
    }

    #[test]
    fn parse_empty_array() {
        let json_str = "[]";
        match parse_json_str(json_str) {
            Ok(JsonValue::Array { values }) => assert_eq!(values.len(), 0),
            _ => panic!("Expected empty array"),
        }
    }

    #[test]
    fn parse_pair() {
        let json_str = r#"
        {
            "pairs": [
                {
                    "x0": 12.5,
                    "y0": 24.5,
                    "x1": 50.5,
                    "y1": 37.5
                },
                {
                    "x0": 12.25,
                    "y0": 22.25,
                    "x1": -17.25,
                    "y1": 3.525e-9
                }
            ]
        }
        "#;

        let result = parse_json_str(json_str).unwrap();

        let pairs = if let JsonValue::Object { kv } = &result {
            kv
        } else {
            panic!("Expected object");
        };

        assert_eq!(pairs.len(), 1);
        let (key, value) = &pairs[0];
        assert_eq!(*key, "pairs");

        let values = if let JsonValue::Array { values } = value {
            values
        } else {
            panic!("Expected array");
        };
        assert_eq!(values.len(), 2);
        let first_expected_pair = JsonValue::Object {
            kv: vec![
                ("x0", JsonValue::Number(12.5f64)),
                ("y0", JsonValue::Number(24.5f64)),
                ("x1", JsonValue::Number(50.5f64)),
                ("y1", JsonValue::Number(37.5f64)),
            ]
            .into_iter()
            .collect(),
        };

        let second_expected_pair = JsonValue::Object {
            kv: vec![
                ("x0", JsonValue::Number(12.25f64)),
                ("y0", JsonValue::Number(22.25f64)),
                ("x1", JsonValue::Number(-17.25f64)),
                ("y1", JsonValue::Number(3.525e-9f64)),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(&values[0], &first_expected_pair, "First pair mismatch");
        assert_eq!(&values[1], &second_expected_pair, "Second pair mismatch");
    }
}
