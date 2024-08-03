pub fn format(json: &str) -> String {
    let value = parse_json_file(json).expect("Invalid json");

    fn serialize(value: &JSONValue) -> String {
        match value {
            JSONValue::Object => "{\n}".to_string(),
            JSONValue::Array(values) => {
                let contents: Vec<String> = values
                    .iter()
                    .map(serialize)
                    .map(|value| format!("\t{}", value))
                    .collect();

                match contents.len() {
                    0 => format!("{}", "[\n]"),
                    _ => format!("[\n{}\n]", contents.join(",\n")),
                }
            }
            JSONValue::Boolean(value) => format!("{}", value),
            JSONValue::Null => "null".to_string(),
        }
    }

    serialize(&value)
}

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

enum JSONValue {
    Object,
    Array(Vec<JSONValue>),
    Boolean(bool),
    Null,
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object,
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            _ => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

#[cfg(test)]
mod json_formatter_tests {
    use super::*;

    #[test]
    fn it_formats_empty_object() {
        let input = "{}";
        let expected = "{\n}";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_empty_array() {
        let input = "[]";
        let expected = "[\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_with_one_boolean_element() {
        let input = "[true]";
        let expected = "[\n\ttrue\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_with_many_boolean_element() {
        let input = "[true,false]";
        let expected = "[\n\ttrue,\n\tfalse\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_of_null_values() {
        let input = "[false, null, null]";
        let expected = "[\n\tfalse,\n\tnull,\n\tnull\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }
}
