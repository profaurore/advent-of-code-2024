use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Position = (usize, usize);
type Offset = (isize, isize);

fn signed_usize_operation(
    is_negative: bool,
) -> fn(usize, usize) -> std::option::Option<usize> {
    if is_negative {
        usize::checked_sub
    } else {
        usize::checked_add
    }
}

fn signed_isize_operation(
    is_negative: bool,
) -> fn(isize, usize) -> std::option::Option<isize> {
    if is_negative {
        isize::checked_sub_unsigned
    } else {
        isize::checked_add_unsigned
    }
}

fn offset_value(
    value: usize,
    offset: isize,
    factor: isize,
    max: usize,
) -> Option<usize> {
    let is_negative_offset = offset.is_negative() ^ factor.is_negative();
    let operation = signed_usize_operation(is_negative_offset);

    operation(value, offset.unsigned_abs() * factor.unsigned_abs())
        .filter(|&value| value <= max)
}

fn offset_position(
    position: Position,
    offset: Offset,
    factor: isize,
    max: Position,
) -> Option<Position> {
    match (
        offset_value(position.0, offset.0, factor, max.0),
        offset_value(position.1, offset.1, factor, max.1),
    ) {
        (Some(v1), Some(v2)) => Some((v1, v2)),
        _ => None,
    }
}

fn pairs(
    values: &'_ HashSet<Position>,
) -> impl Iterator<Item = (Position, Position)> + '_ {
    values.iter().enumerate().flat_map(|(index, &v1)| {
        values.iter().skip(index + 1).map(move |&v2| (v1, v2))
    })
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b
        } else {
            b -= a
        }
    }

    a
}

fn unit_position_offset(p1: Position, p2: Position) -> Option<Offset> {
    let x_op = signed_isize_operation(p1.0 > p2.0);
    let y_op = signed_isize_operation(p1.1 > p2.1);
    let x_diff = p2.0.abs_diff(p1.0);
    let y_diff = p2.1.abs_diff(p1.1);
    let diff_gcd = gcd(x_diff, y_diff);

    match (x_op(0, x_diff / diff_gcd), y_op(0, y_diff / diff_gcd)) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

/// https://adventofcode.com/2024/day/8#part2
pub fn day_08_2() {
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
        .filter_map(|(antenna1, antenna2)| {
            unit_position_offset(antenna1, antenna2).map(move |unit_offset| {
                let positive = (0..).map_while(move |factor| {
                    offset_position(antenna1, unit_offset, factor, max_values)
                });

                let negative = (1..).map_while(move |factor| {
                    offset_position(antenna1, unit_offset, -factor, max_values)
                });

                positive.chain(negative)
            })
        })
        .flatten()
        .collect();

    println!("{}", antinodes.len());
}
