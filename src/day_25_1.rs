use std::fs;

const SIZE: usize = 5;

/// https://adventofcode.com/2024/day/25#part1
pub fn day_25_1() {
    let data = fs::read_to_string("data/day_25.txt").expect("missing file");

    let items = data.split("\n\n");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    items.for_each(|item| {
        let mut data = item
            .split('\n')
            .take_while(|line| !line.is_empty())
            .map(|line| line.as_bytes());

        let first_line = data.next().unwrap();
        let is_lock = first_line[0] == b'#';
        let char = if is_lock { b'#' } else { b'.' };

        let mut heights = [0, 0, 0, 0, 0];

        data.enumerate().for_each(|(height, line)| {
            line.iter().enumerate().for_each(|(pin, &cell)| {
                if cell == char {
                    heights[pin] = height + 1;
                }
            })
        });

        if is_lock {
            locks.push(heights);
        } else {
            heights.iter_mut().for_each(|height| {
                *height = SIZE - *height;
            });
            keys.push(heights);
        }
    });

    let lock_key_fits = locks
        .iter()
        .flat_map(|&lock| {
            keys.iter().filter(move |&key| {
                lock.iter().zip(key).all(|(lock_height, key_height)| {
                    lock_height + key_height <= SIZE
                })
            })
        })
        .count();

    println!("{}", lock_key_fits);
}
