mod json_formatter;

fn main() {
    let input = std::env::args().nth(1).expect("no json given");
    println!("{}", json_formatter::format(&input));
}
