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

pub struct String<'a> {
    pub value: &'a str,
}

impl<'a> Node<'a> for String<'a> {
    fn format(&self) -> std::string::String {
        let value = self.value;
        format!("\"{value}\"")
    }

    fn format_as_child(&self, _tabs: usize) -> std::string::String {
        self.format()
    }

    fn format_as_root(&self) -> std::string::String {
        self.format()
    }
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn it_formats_string() {
        let string = String { value: "foo" };
        let expected = "\"foo\"";
        let result = string.format();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_string_as_child() {
        let string = String { value: "foo" };
        let expected = "\"foo\"";
        let result = string.format_as_child(0);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_string_as_root() {
        let string = String { value: "foo" };
        let expected = "\"foo\"";
        let result = string.format_as_root();

        assert_eq!(expected, result);
    }
}
