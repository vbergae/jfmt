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

use super::Node;

pub struct Object<'a> {
    pub members: Vec<(&'a str, Box<dyn Node<'a> + 'a>)>,
}

impl Object<'_> {
    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

impl<'a> Node<'a> for Object<'a> {
    fn format(&self, indendation: usize) -> String {
        if self.is_empty() {
            return "{}".to_string();
        }

        "".to_string()
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    #[test]
    fn it_formats_an_empty_object() {
        let object = Object { members: vec![] };
        let expected = "{}";
        let result = object.format(0);

        assert_eq!(expected, result);
    }
}
