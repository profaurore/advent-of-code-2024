use std::fs;

use regex::Regex;

/// https://adventofcode.com/2024/day/3#part1
pub fn day_03_2() {
    let data = fs::read_to_string("data/day_03.txt").expect("missing file");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;

    let sum = data.split('\n').filter(|line| !line.is_empty()).fold(
        0,
        |sum, line| {
            regex.captures_iter(line).fold(sum, |sum, capture| {
                if &capture[0] == "do()" {
                    enabled = true;
                    sum
                } else if &capture[0] == "don't()" {
                    enabled = false;
                    sum
                } else {
                    sum + match (
                        capture[1].parse::<u32>(),
                        capture[2].parse::<u32>(),
                    ) {
                        (Ok(a), Ok(b)) if enabled => a * b,
                        _ => 0,
                    }
                }
            })
        },
    );

    println!("{}", sum);
}
