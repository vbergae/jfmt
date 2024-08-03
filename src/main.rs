mod json_formatter;

fn main() {
    let args = Cli::parse();

    println!("{}", json_formatter::format(&args.json));
}

use clap::Parser;

#[derive(Parser)]
struct Cli {
    json: String,
}
