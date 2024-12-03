use std::{fs, iter::zip};

/// https://adventofcode.com/2024/day/1#part1
pub fn day_1_1() {
    let data = fs::read_to_string("data/puzzle_1.txt").expect("missing file");

    let (mut id_list_a, mut id_list_b): (Vec<_>, Vec<_>) = data
        .split('\n')
        .filter_map(|entry| {
            entry.split_once("   ").and_then(|(id_a, id_b)| {
                match (id_a.parse::<usize>(), id_b.parse::<usize>()) {
                    (Ok(id_a), Ok(id_b)) => Some((id_a, id_b)),
                    _ => None,
                }
            })
        })
        .unzip();

    id_list_a.sort();
    id_list_b.sort();

    let total_distance = zip(id_list_a, id_list_b)
        .fold(0, |distance, (id_a, id_b)| distance + id_b.abs_diff(id_a));

    println!("{}", total_distance);
}
