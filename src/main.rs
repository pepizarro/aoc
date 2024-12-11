mod days;

use crate::days::day1;
use crate::days::day2;
use crate::days::day3;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "1" => day1(),
        "2" => day2(),
        "3" => day3(),
        _ => eprintln!("Invalid day: {}", day),
    }
}
