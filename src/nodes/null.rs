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

pub struct Null {}

impl<'a> Node<'a> for Null {
    fn format_as_child(&self, _tabs: usize) -> std::string::String {
        "null".to_string()
    }

    fn format_as_root(&self) -> std::string::String {
        self.format_as_child(0)
    }
}

#[cfg(test)]
mod null_tests {
    use super::*;

    #[test]
    fn test_formats_null_as_child() {
        let null = Null {};
        let result = null.format_as_child(0);

        assert_eq!("null", result);
    }

    #[test]
    fn test_formats_null_as_root() {
        let null = Null {};
        let result = null.format_as_root();

        assert_eq!("null", result);
    }
}
