/*
jfmt
Copyright (C) 2024 - Víctor Berga

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

pub enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

use std::fmt;

impl fmt::Display for JSONValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn serialize(value: &JSONValue, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match value {
                JSONValue::Object(values) => {
                    let contents: Vec<String> = values
                        .iter()
                        .map(|(name, value)| format!("\t\"{}\": {}", name, value))
                        .collect();

                    match contents.len() {
                        0 => write!(f, "{}", "{\n}"),
                        _ => write!(f, "{{\n{}\n}}", contents.join(",\n")),
                    }
                }
                JSONValue::Array(values) => {
                    let contents: Vec<String> =
                        values.iter().map(|value| format!("\t{}", value)).collect();

                    match contents.len() {
                        0 => write!(f, "{}", "[\n]"),
                        _ => write!(f, "[\n{}\n]", contents.join(",\n")),
                    }
                }
                JSONValue::String(string) => write!(f, "\"{}\"", string),
                JSONValue::Number(number) => write!(f, "{}", number),
                JSONValue::Boolean(value) => write!(f, "{}", value),
                JSONValue::Null => write!(f, "{}", "null"),
            }
        }

        serialize(self, f)
    }
}

pub fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            _ => unreachable!(),
        }
    }

    Ok(parse_value(json))
}
