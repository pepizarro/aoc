use regex::Regex;
use std::io::Read;

use super::utils;

pub fn run() {
    let mut file = utils::read_file("inputs/day3.txt");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read the file");

    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    // Replace all matches with an empty string
    let clean = re.replace_all(&s, "   @@@@@@@   ").to_string();
    println!("{}", clean);

    let re2 = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut sum = 0;
    for cap in re2.captures_iter(&clean) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        sum += x * y;
    }
    println!("Day 3: {}", sum);
}
