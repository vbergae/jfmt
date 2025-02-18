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

use crate::parser::parse;

pub fn format(json: &str) -> String {
    match parse(json) {
        Ok(value) => value.format_as_root(),
        Err(error) => format!("[Error] {}", error),
    }
}

#[cfg(test)]
mod json_formatter_tests {
    use super::*;

    #[test]
    fn it_formats_empty_object() {
        let input = "{}";
        let expected = "{}";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_empty_array() {
        let input = "[]";
        let expected = "[]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_with_one_boolean_element() {
        let input = "[true]";
        let expected = "[\n  true\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_with_many_boolean_element() {
        let input = "[true,false]";
        let expected = "[\n  true,\n  false\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_of_null_values() {
        let input = "[false, null, null]";
        let expected = "[\n  false,\n  null,\n  null\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_of_numbers() {
        let input = "[1, 2, -1, 5e3]";
        let expected = "[\n  1,\n  2,\n  -1,\n  5000\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_array_of_strings() {
        let input = "[\"hello\",\"world\"]";
        let expected = "[\n  \"hello\",\n  \"world\"\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_an_object() {
        let input = "{\"name\": \"Nico\",\"foo\": \"bar\"}";
        let expected = "{\n  \u{1b}[1;95m\"name\"\u{1b}[0m: \"Nico\",\n  \u{1b}[1;95m\"foo\"\u{1b}[0m: \"bar\"\n}";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_formats_multidensional_arrays() {
        let input = "[1, [3]]";
        let expected = "[\n  1,\n  [\n    3\n  ]\n]";
        let result = format(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_does_not_panics_when_json_is_bad_formatted() {
        let input = "[1, 2, 3";
        let expected = "[Error]  --> 1:8\n  |\n1 | [1, 2, 3\n  |        ^---\n  |\n  = expected object or array";
        let result = format(input);

        assert_eq!(expected, result);
    }
}
