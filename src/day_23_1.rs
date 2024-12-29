use std::{
    collections::{HashMap, HashSet},
    fs,
};

/// https://adventofcode.com/2024/day/23#part1
pub fn day_23_1() {
    let data = fs::read_to_string("data/day_23.txt").expect("missing file");

    let mut connections: HashMap<_, HashSet<_>> = HashMap::new();
    data.split('\n')
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('-'))
        .for_each(|(a, b)| {
            connections.entry(a).or_default().insert(b);
            connections.entry(b).or_default().insert(a);
        });

    let mut triplets: HashSet<[&str; 3]> = HashSet::new();
    connections.iter().for_each(|(&first, first_connections)| {
        first_connections
            .iter()
            .filter_map(|second| {
                connections
                    .get(second)
                    .map(|second_connections| (second, second_connections))
            })
            .for_each(|(&second, second_connections)| {
                first_connections.intersection(second_connections).for_each(
                    |third| {
                        let mut triplet = [first, second, third];
                        triplet.sort();
                        triplets.insert(triplet);
                    },
                );
            });
    });

    let valid_triplets = triplets
        .iter()
        .filter(|triplet| {
            triplet[0].starts_with('t')
                || triplet[1].starts_with('t')
                || triplet[2].starts_with('t')
        })
        .count();

    println!("{}", valid_triplets);
}
