fn main() {
    let input = std::env::args().nth(1).expect("no json given");
    println!("{}", format_json(&input));
}

fn format_json(json: &str) -> String {
    let ast = parse_json_file(json).expect("Invalid json");

    match ast {
        JSONValue::Object => "{\n}".to_string(),
        JSONValue::Array => "[\n]".to_string(),
    }
}

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

enum JSONValue {
    Object,
    Array,
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    match json.as_rule() {
        Rule::object => Ok(JSONValue::Object),
        Rule::array => Ok(JSONValue::Array),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_formats_empty_object() {
        let input = "{}";
        let expected = "{\n}";
        let result = format_json(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_empty_array() {
        let input = "[]";
        let expected = "[\n]";
        let result = format_json(input);

        assert_eq!(result, expected);
    }
}
