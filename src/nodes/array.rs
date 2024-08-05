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
}

impl fmt::Display for Array<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents: Vec<String> = self
            .values
            .iter()
            .map(|value| format!("\t{}", value))
            .collect();

        match contents.len() {
            0 => write!(f, "{}", "[\n]"),
            _ => write!(f, "[\n{}\n]", contents.join(",\n")),
        }
    }
}
