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

mod formatter;
mod nodes;
mod parser;

fn main() {
    let command = Command::parse();
    let json = &command.contents();

    println!("{}", formatter::format(json));
}

use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Command {
    #[clap(default_value = "-")]
    input: FileOrStdin,
}

impl Command {
    fn contents(&self) -> String {
        let input_clone = self.input.clone();
        let contents = input_clone.contents();

        contents.unwrap_or("".to_string())
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_file_or_stdin() {
        let contents = "{\"foo\": \"bar\"}";
        let temp_file_path = "temp_test_file.txt";
        let mut temp_file = File::create(temp_file_path).unwrap();
        writeln!(temp_file, "{contents}").unwrap();

        let args = Command::parse_from(&["test", temp_file_path]);
        let input = args.contents();
        assert_eq!(input, contents);

        std::fs::remove_file(temp_file_path).unwrap();
    }
}
