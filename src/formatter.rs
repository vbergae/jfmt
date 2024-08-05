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
    let value = parse(json).expect("Invalid json");
    format!("{}", value)
}

#[cfg(test)]
mod json_formatter_tests {
    use super::*;

    #[test]
    fn it_formats_empty_object() {
        let input = "{}";
        let expected = "{\n}";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_empty_array() {
        let input = "[]";
        let expected = "[\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_with_one_boolean_element() {
        let input = "[true]";
        let expected = "[\n\ttrue\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_with_many_boolean_element() {
        let input = "[true,false]";
        let expected = "[\n\ttrue,\n\tfalse\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_of_null_values() {
        let input = "[false, null, null]";
        let expected = "[\n\tfalse,\n\tnull,\n\tnull\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_of_numbers() {
        let input = "[1, 2, -1, 5e3]";
        let expected = "[\n\t1,\n\t2,\n\t-1,\n\t5000\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_array_of_strings() {
        let input = "[\"hello\",\"world\"]";
        let expected = "[\n\t\"hello\",\n\t\"world\"\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_an_object() {
        let input = "{\"name\": \"Nico\",\"foo\": \"bar\"}";
        let expected = "{\n\t\"name\": \"Nico\",\n\t\"foo\": \"bar\"\n}";
        let result = format(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_formats_multidensional_arrays() {
        let input = "[1, [3]]";
        let expected = "[\n\t1,\n\t[\n\t\t3\n\t]\n]";
        let result = format(input);

        assert_eq!(result, expected);
    }
}
