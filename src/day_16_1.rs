use std::{fs, ops::Add};

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        Position {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Direction {
    x: isize,
    y: isize,
}

const DIR_N: Direction = Direction { x: 0, y: -1 };
const DIR_E: Direction = Direction { x: 1, y: 0 };
const DIR_S: Direction = Direction { x: 0, y: 1 };
const DIR_W: Direction = Direction { x: -1, y: 0 };

fn find_cell(map: &[Vec<u8>], search: u8) -> Option<Position> {
    map.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .copied()
            .position(|cell| cell == search)
            .map(|x| Position { x, y })
    })
}

/// https://adventofcode.com/2024/day/16#part1
pub fn day_16_1() {
    let data = fs::read_to_string("data/day_16.txt").expect("missing file");

    let map: Vec<Vec<_>> = data
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let start_pos = match find_cell(&map, b'S') {
        Some(pos) => pos,
        _ => return,
    };
    let end_pos = match find_cell(&map, b'E') {
        Some(pos) => pos,
        _ => return,
    };

    let mut scores: Vec<Vec<_>> = map
        .iter()
        .map(|row| row.iter().map(|_| i32::MAX).collect())
        .collect();
    let mut cells_to_visit = vec![(0, start_pos, DIR_E)];

    while let Some((score, pos, dir)) = cells_to_visit.pop() {
        let current_score = scores[pos.y][pos.x];

        if current_score <= score {
            continue;
        }

        scores[pos.y][pos.x] = score;

        match map[pos.y][pos.x] {
            b'.' | b'S' => {
                [DIR_N, DIR_E, DIR_S, DIR_W].iter().for_each(|&next_dir| {
                    let turn_cost = if next_dir == dir { 0 } else { 1000 };

                    cells_to_visit.push((
                        score + turn_cost + 1,
                        pos.add(next_dir),
                        next_dir,
                    ))
                });
            }

            _ => (),
        }
    }

    println!("{}", scores[end_pos.y][end_pos.x]);
}
