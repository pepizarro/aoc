use std::fs::File;
use std::io::prelude::*;

use std::env;

pub fn run() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs/day1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read the file");

    let lines: Vec<&str> = s.split("\n").collect();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        if nums.len() != 2 {
            continue;
        }
        left.push(nums[0].parse::<i32>().unwrap());
        right.push(nums[1].parse::<i32>().unwrap());
    }

    let left_copy = left.clone();
    let right_copy = right.clone();

    let mut total_distance = 0;
    for _i in 0..left.len() {
        let lmin = left.remove(
            left.iter()
                .position(|&x| x == *left.iter().min().unwrap())
                .unwrap(),
        );
        let rmin = right.remove(
            right
                .iter()
                .position(|&x| x == *right.iter().min().unwrap())
                .unwrap(),
        );

        if lmin > rmin {
            total_distance += lmin - rmin;
        } else {
            total_distance += rmin - lmin;
        }
    }

    println!("Part One: {}", total_distance);
    part_two(&left_copy, &right_copy);
}

fn part_two(left: &[i32], right: &[i32]) {
    let mut similarity = 0;
    for i in left.iter() {
        let count: i32 = right
            .iter()
            .filter(|&x| *x == *i)
            .count()
            .try_into()
            .unwrap();
        similarity += i * count;
    }
    println!("Part Two: {}", similarity);
}
