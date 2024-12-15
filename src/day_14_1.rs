use std::{collections::HashMap, fs};

const MAP_WIDTH: Coord = 101; // test = 11, final = 101
const MAP_HEIGHT: Coord = 103; // test = 7, final = 103
const MAP_WIDTH_HALF: Coord = MAP_WIDTH / 2;
const MAP_HEIGHT_HALF: Coord = MAP_HEIGHT / 2;

type Coord = i64;

struct XY {
    x: Coord,
    y: Coord,
}

fn parse_xy(src: &str) -> Option<XY> {
    src.split_once('=').and_then(|(_, xy_src)| {
        xy_src.split_once(',').and_then(|(x_src, y_src)| {
            match (x_src.parse::<Coord>(), y_src.parse::<Coord>()) {
                (Ok(x), Ok(y)) => Some(XY { x, y }),
                _ => None,
            }
        })
    })
}

struct Robot {
    position: XY,
    velocity: XY,
}

/// https://adventofcode.com/2024/day/14#part1
pub fn day_14_1() {
    let data = fs::read_to_string("data/day_14.txt").expect("missing file");

    let robots =
        data.split('\n')
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
            });

    let mut quadrant_counts: HashMap<(i8, i8), u64> = HashMap::from_iter([
        ((0, 0), 0),
        ((0, 1), 0),
        ((1, 0), 0),
        ((1, 1), 0),
    ]);

    robots.for_each(|robot| {
        let Robot { position, velocity } = robot;

        let x = (position.x + velocity.x * 100).rem_euclid(MAP_WIDTH);
        let y = (position.y + velocity.y * 100).rem_euclid(MAP_HEIGHT);

        if x != MAP_WIDTH_HALF && y != MAP_HEIGHT_HALF {
            quadrant_counts
                .entry(((x > MAP_WIDTH_HALF) as i8, (y > MAP_WIDTH_HALF) as i8))
                .and_modify(|count| *count += 1);
        }
    });

    let safety_factor: u64 = quadrant_counts.values().product();

    println!("{}", safety_factor);
}
