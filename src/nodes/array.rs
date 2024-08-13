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

use crate::nodes::{node::TAB_SPACES, Node};

pub struct Array<'a> {
    pub values: Vec<Box<dyn Node<'a> + 'a>>,
}

impl Array<'_> {
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn format_children(&self, tabs: usize) -> String {
        self.values
            .iter()
            .map(|value| {
                format!(
                    "{}{}",
                    " ".repeat(tabs * TAB_SPACES),
                    value.format_as_child(tabs)
                )
            })
            .reduce(|acc, value| format!("{acc},\n{value}"))
            .unwrap_or("".to_string())
    }
}

impl<'a> Node<'a> for Array<'a> {
    fn format(&self) -> String {
        self.format_as_child(0)
    }

    fn format_as_child(&self, tabs: usize) -> std::string::String {
        if self.is_empty() {
            return "[]".to_string();
        }

        format!(
            "[\n{contents}\n{spaces}]",
            contents = self.format_children(tabs + 1),
            spaces = " ".repeat(tabs * TAB_SPACES)
        )
    }

    fn format_root(&self) -> std::string::String {
        if self.is_empty() {
            return "[]".to_string();
        }

        format!("[\n{contents}\n]", contents = self.format_children(1))
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;
    use crate::nodes::{Boolean, Null};

    #[test]
    fn it_formats_empty_array() {
        let array = Array { values: vec![] };
        let expected = "[]";
        let result = array.format_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_of_nulls() {
        let array = Array {
            values: vec![Box::new(Null {})],
        };
        let expected = "[\n  null\n]";
        let result = array.format_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_a_multidimensional_array_of_nulls() {
        let first_level_array = Array {
            values: vec![Box::new(Null {})],
        };
        let root_array = Array {
            values: vec![Box::new(Null {}), Box::new(first_level_array)],
        };
        let expected = "[\n  null,\n  [\n    null\n  ]\n]";
        let result = root_array.format_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_array_of_booleans() {
        let array = Array {
            values: vec![
                Box::new(Boolean { value: true }),
                Box::new(Boolean { value: false }),
            ],
        };
        let expected = "[\n  true,\n  false\n]";
        let result = array.format_root();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_array_of_multidimensaionl_booleans() {
        let first_level_array = Array {
            values: vec![
                Box::new(Boolean { value: true }),
                Box::new(Boolean { value: false }),
            ],
        };
        let root_array = Array {
            values: vec![Box::new(Null {}), Box::new(first_level_array)],
        };
        let expected = "[\n  null,\n  [\n    true,\n    false\n  ]\n]";
        let result = root_array.format_root();

        assert_eq!(expected, result);
    }
}
