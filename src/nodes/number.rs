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

pub struct Number {
    pub value: f64,
}

impl<'a> Node<'a> for Number {
    fn format(&self) -> String {
        let value = self.value;
        format!("{value}")
    }

    fn format_as_child(&self, _tabs: usize) -> std::string::String {
        self.format()
    }

    fn format_root(&self) -> std::string::String {
        self.format()
    }
}

#[cfg(test)]
mod number_tests {
    use super::*;

    #[test]
    fn it_formats_positive_value() {
        let number = Number { value: 2.0 };
        let expected = "2";
        let result = number.format();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_negative_value() {
        let number = Number { value: -2.0 };
        let expected = "-2";
        let result = number.format();

        assert_eq!(expected, result);
    }
}
