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

use crate::nodes::{Array, Boolean, Node, Null, Number, Object, String};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

pub fn parse(json: &str) -> Result<Box<dyn Node + '_>, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, json)?.next().unwrap();

    Ok(parse_value(json))
}

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

pub fn parse_value<'a>(pair: Pair<'a, Rule>) -> Box<dyn Node<'a> + 'a> {
    match pair.as_rule() {
        Rule::object => Box::new(Object::new(pair)),
        Rule::array => Box::new(Array::new(pair)),
        Rule::string => Box::new(String::new(pair)),
        Rule::number => Box::new(Number::new(pair)),
        Rule::boolean => Box::new(Boolean::new(pair)),
        Rule::null => Box::new(Null::new()),
        _ => unreachable!(),
    }
}

impl<'a> Node<'a> for Object<'a> {}
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

impl<'a> Node<'a> for Array<'a> {}
impl<'a> Array<'a> {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let values = pair.into_inner().map(parse_value).collect();

        Array { values }
    }
}

impl<'a> Node<'a> for String<'a> {}
impl<'a> String<'a> {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.into_inner().next().unwrap().as_str();

        String { value }
    }
}

impl<'a> Node<'a> for Number {}
impl<'a> Number {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.as_str().parse().unwrap();

        Number { value }
    }
}

impl<'a> Node<'a> for Boolean {}
impl<'a> Boolean {
    fn new(pair: Pair<'a, Rule>) -> Self {
        let value = pair.as_str().parse().unwrap();

        Boolean { value }
    }
}

impl<'a> Node<'a> for Null {}
impl Null {
    fn new() -> Self {
        Null {}
    }
}
