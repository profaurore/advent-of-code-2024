use std::{collections::HashMap, fs};

#[derive(Eq, Hash, PartialEq)]
struct StonePosition {
    stone: u64,
    blink: u8,
}

struct StonesCache {
    cache: HashMap<StonePosition, usize>,
    max_blinks: u8,
}

impl StonesCache {
    fn new(max_blinks: u8) -> Self {
        Self {
            cache: HashMap::default(),
            max_blinks,
        }
    }

    fn get_stone_count(&mut self, stone: u64, blink: u8) -> usize {
        if blink == self.max_blinks {
            return 1;
        }

        let position = StonePosition { stone, blink };
        let count = self.cache.get(&position).copied();

        count.unwrap_or_else(|| {
            let count = if stone == 0 {
                self.get_stone_count(1, blink + 1)
            } else {
                let stone_len = stone.ilog10() + 1;
                if stone_len % 2 == 0 {
                    let stone_half = stone_len / 2;
                    let left = stone % 10u64.pow(stone_half);
                    let right = stone / 10u64.pow(stone_half);

                    self.get_stone_count(left, blink + 1)
                        + self.get_stone_count(right, blink + 1)
                } else {
                    self.get_stone_count(stone * 2024, blink + 1)
                }
            };

            self.cache.insert(position, count);

            count
        })
    }
}

/// https://adventofcode.com/2024/day/11#part2
pub fn day_11_2() {
    let data = fs::read_to_string("data/day_11.txt").expect("missing file");

    if let Some((stones_str, _)) = data.split_once('\n') {
        let initial_stones: Box<dyn Iterator<Item = u64>> = Box::new(
            stones_str
                .split(' ')
                .filter_map(|stone_str| stone_str.parse::<u64>().ok()),
        );

        let mut stones_cache = StonesCache::new(75);
        let stones: usize = initial_stones
            .map(|stone| stones_cache.get_stone_count(stone, 0))
            .sum();

        println!("{}", stones);
    }
}
