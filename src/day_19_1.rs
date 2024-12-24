use std::fs;

use regex::Regex;

/// https://adventofcode.com/2024/day/19#part1
pub fn day_19_1() {
    let data = fs::read_to_string("data/day_19.txt").expect("missing file");

    let mut lines = data.split('\n');
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    let _blank = lines.next();
    let designs = lines
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let regex =
        Regex::new(format!("^({})*$", patterns.join("|")).as_str()).unwrap();

    let possible_count = designs
        .iter()
        .filter(|design| regex.is_match(design))
        .count();

    println!("{}", possible_count);
}
