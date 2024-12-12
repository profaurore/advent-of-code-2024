use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Position = (usize, usize);

fn pairs(
    values: &'_ HashSet<Position>,
) -> impl Iterator<Item = (Position, Position)> + '_ {
    values.iter().enumerate().flat_map(|(index, &v1)| {
        values.iter().skip(index + 1).map(move |&v2| (v1, v2))
    })
}

fn lerp(v1: usize, v2: usize, factor: isize, max: usize) -> Option<usize> {
    let is_negative_offset = (v1 > v2) ^ factor.is_negative();
    let operation = if is_negative_offset {
        usize::checked_sub
    } else {
        usize::checked_add
    };

    operation(v1, v2.abs_diff(v1) * factor.unsigned_abs())
        .filter(|&value| value <= max)
}

fn lerp_position(
    p1: Position,
    p2: Position,
    factor: isize,
    max: Position,
) -> Option<Position> {
    match (
        lerp(p1.0, p2.0, factor, max.0),
        lerp(p1.1, p2.1, factor, max.1),
    ) {
        (Some(v1), Some(v2)) => Some((v1, v2)),
        _ => None,
    }
}

/// https://adventofcode.com/2024/day/8#part1
pub fn day_08_1() {
    let data = fs::read_to_string("data/day_08.txt").expect("missing file");

    let num_cols = data.find('\n').unwrap();
    let num_rows = data.len() / (num_cols + 1);
    let max_values = (num_cols - 1, num_rows - 1);

    let mut antennas: HashMap<u8, HashSet<(usize, usize)>> = HashMap::new();
    data.split('\n').enumerate().for_each(|(row, line)| {
        line.as_bytes().iter().enumerate().for_each(|(col, &char)| {
            if char != b'.' {
                antennas.entry(char).or_default().insert((col, row));
            }
        })
    });

    let antinodes: HashSet<Position> = antennas
        .values()
        .flat_map(pairs)
        .flat_map(|(antenna1, antenna2)| {
            [
                lerp_position(antenna1, antenna2, 2, max_values),
                lerp_position(antenna1, antenna2, -1, max_values),
            ]
        })
        .flatten()
        .collect();

    println!("{:?}", antinodes);

    println!("{}", antinodes.len());
}
