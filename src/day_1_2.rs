use std::fs;

/// https://adventofcode.com/2024/day/1#part2
pub fn day_1_2() {
    let data = fs::read_to_string("data/day_1.txt").expect("missing file");

    let (id_list_a, id_list_b): (Vec<_>, Vec<_>) = data
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

    let similarity_score = id_list_a.iter().fold(0, |score, id_a| {
        score
            + id_a
                * id_list_b.iter().fold(0, |count, id_b| {
                    if id_b == id_a {
                        count + 1
                    } else {
                        count
                    }
                })
    });

    println!("{}", similarity_score);
}
