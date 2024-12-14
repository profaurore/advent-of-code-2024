use std::fs;

/// https://adventofcode.com/2024/day/11#part1
pub fn day_11_1() {
    let data = fs::read_to_string("data/day_11.txt").expect("missing file");

    if let Some((stones_str, _)) = data.split_once('\n') {
        let stones: Vec<_> = (0..25).fold(
            stones_str
                .split(' ')
                .filter_map(|stone_str| stone_str.parse::<u64>().ok())
                .collect(),
            |stones, _| {
                stones
                    .iter()
                    .flat_map(|&stone| {
                        if stone == 0 {
                            vec![1]
                        } else {
                            let stone_str = stone.to_string();

                            if stone_str.len() % 2 == 0 {
                                let (left, right) =
                                    stone_str.split_at(stone_str.len() / 2);

                                vec![
                                    left.parse::<u64>().unwrap(),
                                    right.parse::<u64>().unwrap(),
                                ]
                            } else {
                                vec![stone * 2024]
                            }
                        }
                    })
                    .collect()
            },
        );

        println!("{}", stones.len());
    }
}
