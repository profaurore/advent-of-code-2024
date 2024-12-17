use std::fs;

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
    let next_robot_position = add_move(robot_position, movement);

    let mut check_position = next_robot_position;

    loop {
        match map[check_position.1][check_position.0] {
            b'O' => check_position = add_move(check_position, movement),
            b'#' => return robot_position,
            _ => {
                map[robot_position.1][robot_position.0] = b'.';
                map[next_robot_position.1][next_robot_position.0] = b'@';
                if check_position != next_robot_position {
                    map[check_position.1][check_position.0] = b'O';
                }

                return next_robot_position;
            }
        }
    }
}

// fn print_map(map: &[Vec<u8>]) {
//     map.iter().for_each(|row| {
//         row.iter().for_each(|cell| {
//             print!(
//                 "{}",
//                 match cell {
//                     b'#' => '#',
//                     b'O' => 'O',
//                     b'@' => '@',
//                     _ => '.',
//                 }
//             )
//         });

//         println!();
//     });

//     println!();
// }

/// https://adventofcode.com/2024/day/15#part1
pub fn day_15_1() {
    let data = fs::read_to_string("data/day_15.txt").expect("missing file");
    let mut lines = data.split('\n');

    let mut map: Vec<Vec<u8>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
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
            // print_map(&map);

            robot_position = move_dir(movement, robot_position, &mut map);

            // println!("{:?}", movement);
        });
    }

    // print_map(&map);

    let total = map.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(acc, |acc, (x, &cell)| {
            acc + if cell == b'O' { 100 * y + x } else { 0 }
        })
    });

    println!("{}", total);
}
