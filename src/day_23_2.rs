use std::{
    collections::{HashMap, HashSet},
    fs,
};

/// https://adventofcode.com/2024/day/23#part2
pub fn day_23_2() {
    let data = fs::read_to_string("data/day_23.txt").expect("missing file");

    let mut connections: HashMap<_, HashSet<_>> = HashMap::new();
    data.split('\n')
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('-'))
        .for_each(|(a, b)| {
            connections.entry(a).or_default().insert(b);
            connections.entry(b).or_default().insert(a);
        });

    let mut networks: Vec<Vec<_>> = Vec::new();

    connections
        .iter()
        .for_each(|(&computer, computer_connections)| {
            let mut computers_with_networks: HashSet<&str> = HashSet::new();

            networks.iter_mut().for_each(|network| {
                if network.iter().all(|network_computer| {
                    computer_connections.contains(network_computer)
                }) {
                    network.push(computer);
                    network.sort();
                    computers_with_networks.extend(network.iter());
                }
            });

            networks.extend(
                computer_connections
                    .difference(&computers_with_networks)
                    .map(|connected_computer| {
                        Vec::from_iter([computer, connected_computer])
                    }),
            );

            networks.sort_by_key(|n| n.join(","));
            networks.dedup_by_key(|n| n.join(","));
        });

    let largest_network = networks.iter().max_by_key(|n| n.len()).unwrap();

    println!("{}", largest_network.join(","));
}
