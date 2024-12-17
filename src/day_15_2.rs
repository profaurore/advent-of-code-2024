use std::{collections::vec_deque::VecDeque, fs};

const SHOW_MAPS: bool = false;

type Movement = (isize, isize);
type Position = (usize, usize);

fn add_move(position: Position, movement: Movement) -> Position {
    (
        position.0.saturating_add_signed(movement.0),
        position.1.saturating_add_signed(movement.1),
    )
}

fn move_dir(
    movement: Movement,
    robot_position: Position,
    map: &mut [Vec<u8>],
) -> Position {
    let mut pushed_objects = Vec::new();
    let mut objects_to_move: VecDeque<Vec<_>> = VecDeque::new();

    objects_to_move.push_back(vec![robot_position]);

    while let Some(object_to_move) = objects_to_move.pop_front() {
        let next_positions: Vec<_> = object_to_move
            .iter()
            .map(|&position| add_move(position, movement))
            .collect();

        for next_position in &next_positions {
            match map[next_position.1][next_position.0] {
                b'#' => return robot_position,

                b'[' if !object_to_move.contains(next_position)
                    && objects_to_move.iter().all(|object| {
                        object
                            .first()
                            .is_none_or(|&first| first != *next_position)
                    }) =>
                {
                    objects_to_move.push_back(vec![
                        *next_position,
                        (next_position.0 + 1, next_position.1),
                    ])
                }

                b']' if !object_to_move
                    .contains(&(next_position.0 - 1, next_position.1))
                    && objects_to_move.iter().all(|object| {
                        object.first().is_none_or(|&first| {
                            first != (next_position.0 - 1, next_position.1)
                        })
                    }) =>
                {
                    objects_to_move.push_back(vec![
                        (next_position.0 - 1, next_position.1),
                        *next_position,
                    ])
                }

                _ => (),
            }
        }

        pushed_objects.push((object_to_move, next_positions));
    }

    pushed_objects
        .iter()
        .rev()
        .for_each(|(positions, next_positions)| {
            let values: Vec<_> = positions
                .iter()
                .map(|position| {
                    let value = map[position.1][position.0];
                    map[position.1][position.0] = b'.';
                    value
                })
                .collect();

            next_positions.iter().zip(values).for_each(
                |(next_position, value)| {
                    map[next_position.1][next_position.0] = value;
                },
            );
        });

    add_move(robot_position, movement)
}

fn print_map(map: &[Vec<u8>]) {
    map.iter().for_each(|row| {
        row.iter().for_each(|&cell| print!("{}", cell as char));

        println!();
    });

    println!();
}

/// https://adventofcode.com/2024/day/15#part2
pub fn day_15_2() {
    let data = fs::read_to_string("data/day_15.txt").expect("missing file");
    let mut lines = data.split('\n');

    let mut map: Vec<Vec<u8>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .flat_map(|&cell| match cell {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    _ => [b'.', b'.'],
                })
                .collect()
        })
        .collect();

    let movements = lines.flat_map(|line| {
        line.as_bytes()
            .iter()
            .filter_map(|movement| match movement {
                b'^' => Some((0, -1)),
                b'>' => Some((1, 0)),
                b'v' => Some((0, 1)),
                b'<' => Some((-1, 0)),
                _ => None,
            })
    });

    if let Some(mut robot_position) =
        map.iter().enumerate().find_map(|(y, row)| {
            row.iter().position(|&cell| cell == b'@').map(|x| (x, y))
        })
    {
        movements.for_each(|movement| {
            if SHOW_MAPS {
                print_map(&map);
            }

            robot_position = move_dir(movement, robot_position, &mut map);

            if SHOW_MAPS {
                println!("{:?}", movement);
            }
        });
    }

    if SHOW_MAPS {
        print_map(&map);
    }

    let total = map.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(acc, |acc, (x, &cell)| {
            acc + if cell == b'[' { 100 * y + x } else { 0 }
        })
    });

    println!("{}", total);
}
