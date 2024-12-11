use std::fs::File;
use std::io::prelude::*;

use std::env;

fn open_file() -> File {
    // Create a path to the desired file
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs/day2.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    }
}

fn is_safe_decreasing(levels: Vec<i32>) -> bool {
    let mut safe = true;
    for i in 0..levels.len() - 1 {
        if levels[i] < levels[i + 1] {
            safe = false;
            break;
        }
        if levels[i] - levels[i + 1] > 3 || levels[i] - levels[i + 1] < 1 {
            safe = false;
            break;
        }
    }
    safe
}

fn is_safe_increasing(levels: Vec<i32>) -> bool {
    let mut safe = true;
    for i in 0..levels.len() - 1 {
        if levels[i] > levels[i + 1] {
            safe = false;
            break;
        }
        if levels[i + 1] - levels[i] > 3 || levels[i + 1] - levels[i] < 1 {
            safe = false;
            break;
        }
    }
    safe
}

fn is_safe_decreasing_tolerance(mut levels: Vec<i32>, first: bool) -> bool {
    println!("Checking decreasing for: {:?}", levels);
    for i in 0..levels.len() - 1 {
        if levels[i] < levels[i + 1] {
            if !first {
                return false;
            }
            let mut levels_copy = levels.clone();
            levels_copy.remove(i);
            levels.remove(i + 1);
            return is_safe_decreasing_tolerance(levels, false)
                || is_safe_decreasing_tolerance(levels_copy, false);
        }
        if levels[i] - levels[i + 1] > 3 || levels[i] - levels[i + 1] < 1 {
            if !first {
                return false;
            }
            let mut levels_copy = levels.clone();
            levels_copy.remove(i);
            levels.remove(i + 1);
            return is_safe_decreasing_tolerance(levels, false)
                || is_safe_decreasing_tolerance(levels_copy, false);
        }
    }

    println!("{:?} Is safe decreasing with 1 level tolerance", levels);
    true
}

fn is_safe_increasing_tolerance(mut levels: Vec<i32>, first: bool) -> bool {
    println!("Checking increasing for: {:?}, {}", levels, first);
    for i in 0..levels.len() - 1 {
        if levels[i] > levels[i + 1] {
            if !first {
                return false;
            }
            let mut levels_copy = levels.clone();
            levels_copy.remove(i);
            levels.remove(i + 1);
            return is_safe_increasing_tolerance(levels, false)
                || is_safe_increasing_tolerance(levels_copy, false);
        }

        if levels[i + 1] - levels[i] > 3 || levels[i + 1] - levels[i] < 1 {
            if !first {
                return false;
            }
            let mut levels_copy = levels.clone();
            levels_copy.remove(i);
            levels.remove(i + 1);
            return is_safe_increasing_tolerance(levels, false)
                || is_safe_increasing_tolerance(levels_copy, false);
        }
    }

    println!("{:?} Is safe increasing with 1 level tolerance", levels);
    true
}

fn count_safe_reports_tolerance(reports: Vec<&str>) -> i32 {
    let mut safe_reports = 0;

    for report in reports {
        let levels: Vec<&str> = report.split_whitespace().collect();
        if levels.is_empty() {
            continue;
        }

        let mut levels_int: Vec<i32> = Vec::new();
        for level in levels {
            levels_int.push(level.parse().unwrap());
        }

        if is_safe_decreasing_tolerance(levels_int.clone(), true)
            || is_safe_increasing_tolerance(levels_int.clone(), true)
        {
            safe_reports += 1;
            println!("{:?} is safe", report)
        }
    }
    safe_reports
}

fn count_safe_reports(reports: Vec<&str>) -> i32 {
    let mut safe_reports = 0;

    for report in reports {
        let levels: Vec<&str> = report.split_whitespace().collect();
        if levels.is_empty() {
            continue;
        }

        let mut levels_int: Vec<i32> = Vec::new();
        for level in levels {
            levels_int.push(level.parse().unwrap());
        }

        if is_safe_decreasing(levels_int.clone()) || is_safe_increasing(levels_int.clone()) {
            safe_reports += 1;
        }
    }
    safe_reports
}

pub fn run() {
    let mut file = open_file();

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("couldn't read file to string");

    let reports: Vec<&str> = s.split("\n").collect();

    println!("Safe reports: {}", count_safe_reports(reports.clone()));
    println!(
        "Safe reports with tolerance: {}",
        count_safe_reports_tolerance(reports)
    );
}
