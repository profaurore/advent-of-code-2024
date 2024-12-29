use std::fs;

fn mix_and_prune(secret_number: u64, other_number: u64) -> u64 {
    (secret_number ^ other_number) % 16777216
}

/// https://adventofcode.com/2024/day/22#part1
pub fn day_22_1() {
    let data = fs::read_to_string("data/day_22.txt").expect("missing file");

    let secret_numbers = data
        .split('\n')
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.parse::<u64>().ok());

    let sum = secret_numbers
        .map(|secret_number| {
            (0..2000).fold(secret_number, |secret_number, _| {
                let secret_number =
                    mix_and_prune(secret_number, secret_number * 64);
                let secret_number =
                    mix_and_prune(secret_number, secret_number / 32);

                mix_and_prune(secret_number, secret_number * 2048)
            })
        })
        .sum::<u64>();

    println!("{}", sum);
}
