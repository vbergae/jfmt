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

use crate::nodes::Node;
use std::fmt;

pub struct Array<'a> {
    pub values: Vec<Box<dyn Node<'a> + 'a>>,
    pub indendation: usize,
}

impl<'a> Node<'a> for Array<'a> {
    fn format(&self, indendation: usize) -> String {
        let contents = self
            .values
            .iter()
            .fold("".to_string(), |acc, value| value.format(indendation + 1));

        format!("[\n{:?}\n]", contents)
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;
    use crate::nodes::Null;

    #[test]
    fn test_formats_an_array_of_nulls() {
        let array = Array {
            values: vec![Box::new(Null {})],
            indendation: 0,
        };
        let result = array.format(0);
        let expected = "[\n  \"null\"\n]";

        assert_eq!(expected, result);
    }
}
