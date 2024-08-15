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

use crate::nodes::node::TAB_SPACES;

use super::Node;

pub struct Object<'a> {
    pub members: Vec<(&'a str, Box<dyn Node<'a> + 'a>)>,
}

impl Object<'_> {
    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    fn format_children(&self, tabs: usize) -> String {
        self.members
            .iter()
            .map(|member| {
                let indendation = " ".repeat(tabs * TAB_SPACES);
                let attribute = member.0;
                let value = member.1.format_as_child(tabs);

                format!("{indendation}\"{attribute}\": {value}")
            })
            .reduce(|acc, member| format!("{acc},\n{member}"))
            .unwrap_or("".to_string())
    }
}

impl<'a> Node<'a> for Object<'a> {
    fn format_as_child(&self, tabs: usize) -> std::string::String {
        if self.is_empty() {
            return "{}".to_string();
        }

        format!(
            "{{\n{contents}\n{spaces}}}",
            contents = self.format_children(tabs + 1),
            spaces = " ".repeat(tabs * TAB_SPACES)
        )
    }

    fn format_as_root(&self) -> std::string::String {
        if self.is_empty() {
            return "{}".to_string();
        }

        let contents = self.format_children(1);
        format!("{{\n{contents}\n}}")
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    use crate::nodes::{Number, String};

    #[test]
    fn it_formats_an_empty_object() {
        let object = Object { members: vec![] };
        let expected = "{}";
        let result = object.format_as_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_object() {
        let object = Object {
            members: vec![("member", Box::new(String { value: "value" }))],
        };
        let expected = "{\n  \"member\": \"value\"\n}";
        let result = object.format_as_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_a_multidimensional_object() {
        let first_level_object = Object {
            members: vec![("member", Box::new(String { value: "value" }))],
        };
        let object = Object {
            members: vec![
                ("member", Box::new(String { value: "value" })),
                ("number", Box::new(Number { value: 2.0 })),
                ("child", Box::new(first_level_object)),
            ],
        };
        let expected = "{\n  \"member\": \"value\",\n  \"number\": 2,\n  \"child\": {\n    \"member\": \"value\"\n  }\n}";
        let result = object.format_as_root();

        assert_eq!(expected, result);
    }
}
