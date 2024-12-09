use std::{collections::HashSet, fs};

enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn add(&self, &(x, y): &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::Right => (x.wrapping_add(1), y),
            Direction::Down => (x, y.wrapping_add(1)),
            Direction::Left => (x.wrapping_sub(1), y),
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_at_coords_builder<'a>(
    bytes: &'a [u8],
    num_cols: usize,
) -> impl Fn((usize, usize)) -> Option<&'a u8> {
    let cols_range = 0..num_cols;

    move |(x, y)| {
        // Rows range is checked by the bytes bounds.
        if cols_range.contains(&x) {
            y.checked_mul(num_cols + 1)
                .and_then(|y| x.checked_add(y))
                .and_then(|index| bytes.get(index))
        } else {
            None
        }
    }
}

/// https://adventofcode.com/2024/day/6#part1
pub fn day_6_1() {
    let data = fs::read_to_string("data/day_6.txt").expect("missing file");
    let bytes = data.as_bytes();

    let mut positions = HashSet::new();

    if let Some(num_cols) = data.find('\n') {
        if let Some(location) = data.find('^') {
            let get_at_coords = get_at_coords_builder(bytes, num_cols);

            let cols_offset = num_cols + 1;
            let mut position = (location % cols_offset, location / cols_offset);
            let mut direction = Direction::Up;

            loop {
                positions.insert(position);

                let next_position = direction.add(&position);
                let next_cell = get_at_coords(next_position);

                match next_cell {
                    Some(b'#') => direction = direction.next(),
                    Some(_) => position = next_position,
                    None => break,
                }
            }
        }
    }

    println!("{}", positions.len());
}
