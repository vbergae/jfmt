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

use core::fmt;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

pub fn parse_json_file(file: &str) -> Result<Box<dyn Node + '_>, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    Ok(parse_value(json))
}

pub trait Node<'a>: fmt::Display {}

pub struct Object<'a> {
    pub members: Vec<(&'a str, Box<dyn Node<'a> + 'a>)>,
}

impl<'a> Node<'a> for Object<'a> {}

impl fmt::Display for Object<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let contents: Vec<String> = self
            .members
            .iter()
            .map(|(name, value)| format!("\t\"{}\": {}", name, value))
            .collect();

        match contents.len() {
            0 => write!(f, "{}", "{\n}"),
            _ => write!(f, "{{\n{}\n}}", contents.join(",\n")),
        }
    }
}

impl<'a> Object<'a> {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let attributes = pair
            .into_inner()
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
            .collect();

        Object {
            members: attributes,
        }
    }
}

pub struct Array<'a> {
    pub values: Vec<Box<dyn Node<'a> + 'a>>,
}

impl<'a> Array<'a> {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let values = pair.into_inner().map(parse_value).collect();

        Array { values }
    }
}

impl<'a> Node<'a> for Array<'a> {}
impl fmt::Display for Array<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents: Vec<String> = self
            .values
            .iter()
            .map(|value| format!("\t{}", value))
            .collect();

        match contents.len() {
            0 => write!(f, "{}", "[\n]"),
            _ => write!(f, "[\n{}\n]", contents.join(",\n")),
        }
    }
}

pub struct StringNode<'a> {
    value: &'a str,
}
impl<'a> StringNode<'a> {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.into_inner().next().unwrap().as_str();

        StringNode { value }
    }
}
impl<'a> Node<'a> for StringNode<'a> {}
impl fmt::Display for StringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

pub struct Number {
    value: f64,
}
impl<'a> Number {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.as_str().parse().unwrap();

        Number { value }
    }
}
impl<'a> Node<'a> for Number {}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct Boolean {
    value: bool,
}
impl<'a> Boolean {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.as_str().parse().unwrap();

        Boolean { value }
    }
}
impl<'a> Node<'a> for Boolean {}
impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct Null {}
impl Null {
    fn new() -> Self {
        Null {}
    }
}
impl<'a> Node<'a> for Null {}
impl fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "null")
    }
}

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

fn parse_value<'a>(pair: Pair<'a, Rule>) -> Box<dyn Node<'a> + 'a> {
    match pair.as_rule() {
        Rule::object => Box::new(Object::new(pair)),
        Rule::array => Box::new(Array::new(pair)),
        Rule::string => Box::new(StringNode::new(pair)),
        Rule::number => Box::new(Number::new(pair)),
        Rule::boolean => Box::new(Boolean::new(pair)),
        Rule::null => Box::new(Null::new()),
        _ => unreachable!(),
    }
}
