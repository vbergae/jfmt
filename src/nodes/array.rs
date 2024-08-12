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

    fn format_contents(&self, with_indendation: usize) -> String {
        self.values
            .iter()
            .map(|value| value.format(with_indendation + 1))
            .reduce(|acc, value| format!("{acc},\n{value}"))
            .unwrap_or("".to_string())
    }
}

impl<'a> Node<'a> for Array<'a> {
    fn format(&self, indendation: usize) -> String {
        if self.is_empty() {
            return "[]".to_string();
        }

        let contents = self.format_contents(indendation);
        let spaces = " ".repeat(indendation * TAB_SPACES);

        format!("{spaces}[\n{contents}\n{spaces}]")
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
        let result = array.format(0);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_of_nulls() {
        let array = Array {
            values: vec![Box::new(Null {})],
        };
        let expected = "[\n  null\n]";
        let result = array.format(0);

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
        let result = root_array.format(0);

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
        let result = array.format(0);

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
        let result = root_array.format(0);

        assert_eq!(expected, result);
    }
}
