use std::{collections::HashMap, fs};

use regex::Regex;

fn get_arrangements_count<'a>(
    patterns: &Vec<&str>,
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&count) = cache.get(design) {
        return count;
    }

    let count = if design.is_empty() {
        1
    } else {
        patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|pattern| {
                get_arrangements_count(
                    patterns,
                    &design[pattern.len()..],
                    cache,
                )
            })
            .sum()
    };

    cache.insert(design, count);

    count
}

/// https://adventofcode.com/2024/day/19#part2
pub fn day_19_2() {
    let data = fs::read_to_string("data/day_19.txt").expect("missing file");

    let mut lines = data.split('\n');
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    let _blank = lines.next();
    let designs = lines
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let regex =
        Regex::new(format!("^({})*$", patterns.join("|")).as_str()).unwrap();

    let mut cache = HashMap::new();

    let possible_count: usize = designs
        .iter()
        .filter(|design| regex.is_match(design))
        .map(|design| get_arrangements_count(&patterns, design, &mut cache))
        .sum();

    println!("{}", possible_count);
}
