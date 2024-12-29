use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn mix_and_prune(secret_number: i64, other_number: i64) -> i64 {
    (secret_number ^ other_number) % 16777216
}

#[derive(Clone, Copy, Default)]
struct SecretNumberWithDelta {
    number: i64,
    delta: i64,
}

struct SecretNumberIter {
    next_value: SecretNumberWithDelta,
}

impl SecretNumberIter {
    fn new(init: i64) -> Self {
        Self {
            next_value: SecretNumberWithDelta {
                number: init,
                delta: 0,
            },
        }
    }
}

impl Iterator for SecretNumberIter {
    type Item = SecretNumberWithDelta;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.next_value;
        let number = value.number;

        let mut next_number = mix_and_prune(number, number * 64);
        next_number = mix_and_prune(next_number, next_number / 32);
        next_number = mix_and_prune(next_number, next_number * 2048);

        self.next_value = SecretNumberWithDelta {
            number: next_number,
            delta: (next_number % 10) - (number % 10),
        };

        Some(value)
    }
}

/// https://adventofcode.com/2024/day/22#part2
pub fn day_22_2() {
    let data = fs::read_to_string("data/day_22.txt").expect("missing file");

    let secret_numbers = data
        .split('\n')
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.parse::<i64>().ok());

    let mut change_values: HashMap<_, i64> = HashMap::new();

    secret_numbers.for_each(|init_number| {
        let mut secret_numbers = SecretNumberIter::new(init_number);
        let mut seen_changes = HashSet::new();

        let mut prev = [
            secret_numbers.next().unwrap(),
            secret_numbers.next().unwrap(),
            secret_numbers.next().unwrap(),
        ];

        for _ in 0..=(2000 - prev.len()) {
            let value = secret_numbers.next().unwrap();

            if seen_changes.insert([
                prev[0].delta,
                prev[1].delta,
                prev[2].delta,
                value.delta,
            ]) {
                *change_values
                    .entry([
                        prev[0].delta,
                        prev[1].delta,
                        prev[2].delta,
                        value.delta,
                    ])
                    .or_default() += value.number % 10;
            }

            prev = [prev[1], prev[2], value];
        }
    });

    let (_, &max_bananas) =
        change_values.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    println!("{}", max_bananas);
}
