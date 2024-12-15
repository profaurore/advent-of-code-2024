use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
    str::FromStr,
};

const MAP_WIDTH: usize = 101;
const MAP_HEIGHT: usize = 103;
const EMPTY: isize = -1;

type Coord = usize;
type Dir = isize;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct XY<TUnit>
where
    TUnit: Copy + Clone + Eq + Hash + PartialEq,
{
    x: TUnit,
    y: TUnit,
}

type Position = XY<Coord>;
type Velocity = XY<Dir>;

fn parse_xy<TUnit>(src: &str) -> Option<XY<TUnit>>
where
    TUnit: Copy + Clone + Eq + Hash + PartialEq + FromStr,
{
    src.split_once('=').and_then(|(_, xy_src)| {
        xy_src.split_once(',').and_then(|(x_src, y_src)| {
            match (x_src.parse::<TUnit>(), y_src.parse::<TUnit>()) {
                (Ok(x), Ok(y)) => Some(XY { x, y }),
                _ => None,
            }
        })
    })
}

struct Robot {
    position: Position,
    velocity: Velocity,
}

fn print_map(cells_with_robots: &HashSet<Position>) {
    println!("+{}+", "-".repeat(MAP_WIDTH));

    (0..MAP_HEIGHT).for_each(|y| {
        print!("|");

        (0..MAP_WIDTH).for_each(|x| {
            if cells_with_robots.contains(&XY { x, y }) {
                print!("*");
            } else {
                print!(" ");
            }
        });

        println!("|");
    });

    println!("+{}+", "-".repeat(MAP_WIDTH));
}

fn map_regions(map: &mut [Vec<isize>], cells_with_robots: &HashSet<Position>) {
    map.iter_mut().for_each(|row| row.fill(EMPTY));

    let mut last_region_id = -1;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let cell = XY { x, y };
            if map[y][x] == EMPTY && cells_with_robots.contains(&cell) {
                last_region_id += 1;

                let mut remaining = Vec::new();
                remaining.push(cell);

                while let Some(cell) = remaining.pop() {
                    let XY { x, y } = cell;

                    if let Some(EMPTY) =
                        map.get(y).and_then(|row| row.get(x).copied())
                    {
                        if map[y][x] == EMPTY
                            && cells_with_robots.contains(&cell)
                        {
                            map[y][x] = last_region_id;

                            remaining.extend(
                                [
                                    XY {
                                        x: x.wrapping_sub(1),
                                        y: y.wrapping_sub(1),
                                    },
                                    XY {
                                        x: x.wrapping_sub(1),
                                        y: y.saturating_add(1),
                                    },
                                    XY {
                                        x: x.saturating_add(1),
                                        y: y.wrapping_sub(1),
                                    },
                                    XY {
                                        x: x.saturating_add(1),
                                        y: y.saturating_add(1),
                                    },
                                    XY {
                                        x: x.wrapping_sub(1),
                                        y,
                                    },
                                    XY {
                                        x,
                                        y: y.saturating_add(1),
                                    },
                                    XY {
                                        x,
                                        y: y.wrapping_sub(1),
                                    },
                                    XY {
                                        x: x.saturating_add(1),
                                        y,
                                    },
                                ]
                                .iter(),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn has_christmas_tree(map: &[Vec<isize>], num_robots: usize) -> bool {
    let mut region_counts: HashMap<isize, usize> = HashMap::new();

    map.iter().for_each(|row| {
        row.iter().for_each(|&region_id| {
            if region_id != EMPTY {
                *region_counts.entry(region_id).or_default() += 1;
            }
        })
    });

    let mut regions: Vec<_> = region_counts.values().copied().collect();
    regions.sort();

    let total_in_top_regions: usize = regions.iter().rev().take(4).sum();

    total_in_top_regions > num_robots / 4
}

/// https://adventofcode.com/2024/day/14#part2
pub fn day_14_2() {
    let data = fs::read_to_string("data/day_14.txt").expect("missing file");

    let mut robots: Vec<_> = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|src| {
            src.split_once(' ')
                .and_then(|(position_src, velocity_src)| {
                    match (parse_xy(position_src), parse_xy(velocity_src)) {
                        (Some(position), Some(velocity)) => {
                            Some(Robot { position, velocity })
                        }
                        _ => None,
                    }
                })
        })
        .collect();

    let mut region_map = vec![vec![EMPTY; MAP_WIDTH]; MAP_HEIGHT];

    for i in 1..=1_000_000 {
        robots.iter_mut().for_each(|robot| {
            let Robot { position, velocity } = robot;

            position.x = (velocity.x.saturating_add_unsigned(position.x))
                .rem_euclid(MAP_WIDTH as isize)
                .unsigned_abs();
            position.y = (velocity.y.saturating_add_unsigned(position.y))
                .rem_euclid(MAP_HEIGHT as isize)
                .unsigned_abs();
        });

        let cells_with_robots: HashSet<_> =
            robots.iter().map(|robot| robot.position).collect();

        map_regions(&mut region_map, &cells_with_robots);

        if has_christmas_tree(&region_map, robots.len()) {
            print_map(&cells_with_robots);
            println!("{}", i);
            break;
        }
    }
}
