use std::fs;

use regex::Regex;

/// https://adventofcode.com/2024/day/3#part1
pub fn day_3_1() {
    let data = fs::read_to_string("data/day_3.txt").expect("missing file");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum = data.split('\n').filter(|line| !line.is_empty()).fold(
        0,
        |sum, line| {
            regex.captures_iter(line).fold(sum, |sum, capture| {
                sum + match (
                    capture[1].parse::<u32>(),
                    capture[2].parse::<u32>(),
                ) {
                    (Ok(a), Ok(b)) => a * b,
                    _ => 0,
                }
            })
        },
    );

    println!("{}", sum);
}
