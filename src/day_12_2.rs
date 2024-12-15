use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    ops::AddAssign,
};

const EMPTY: i32 = -1;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum OutDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Segment {
    p1: Coord,
    p2: Coord,
    dir: OutDirection,
}

impl Segment {
    fn new(
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        dir: OutDirection,
    ) -> Self {
        Self {
            p1: Coord::new(x1, y1),
            p2: Coord::new(x2, y2),
            dir,
        }
    }

    fn from_coords(p1: Coord, p2: Coord, dir: OutDirection) -> Self {
        Self { p1, p2, dir }
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?}", self.p1, self.p2)
    }
}

#[derive(Default)]
struct PerimeterSet(HashSet<Segment>);

impl PerimeterSet {
    fn extend<'a, T: IntoIterator<Item = &'a Segment>>(&mut self, iter: T) {
        iter.into_iter().for_each(|&segment| {
            self.insert(segment);
        });
    }

    fn insert(&mut self, segment: Segment) {
        let dir = segment.dir;
        let mut new_segment = segment;

        loop {
            if let Some(adjacent) = self
                .0
                .iter()
                .find(|segment| {
                    segment.dir == dir && segment.p2 == new_segment.p1
                })
                .copied()
            {
                self.0.remove(&adjacent);
                new_segment =
                    Segment::from_coords(adjacent.p1, new_segment.p2, dir);
            } else if let Some(adjacent) = self
                .0
                .iter()
                .find(|segment| {
                    segment.dir == dir && new_segment.p2 == segment.p1
                })
                .copied()
            {
                self.0.remove(&adjacent);
                new_segment =
                    Segment::from_coords(new_segment.p1, adjacent.p2, dir);
            } else {
                break;
            }
        }

        self.0.insert(new_segment);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Segment> {
        self.0.iter()
    }
}

impl AddAssign for PerimeterSet {
    fn add_assign(&mut self, rhs: Self) {
        self.extend(rhs.iter());
    }
}

impl Debug for PerimeterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn in_region(r: usize, c: usize, region_id: i32, regions: &[Vec<i32>]) -> bool {
    regions
        .get(r)
        .and_then(|row| row.get(c))
        .is_some_and(|&cell_id| cell_id == region_id)
}

fn cell_fences(r: usize, c: usize, regions: &[Vec<i32>]) -> PerimeterSet {
    let region_id = regions[r][c];

    let mut fences = PerimeterSet::default();

    if !in_region(r, c.wrapping_sub(1), region_id, regions) {
        fences.insert(Segment::new(
            c,
            r,
            c,
            r.saturating_add(1),
            OutDirection::Left,
        ));
    }
    if !in_region(r, c.saturating_add(1), region_id, regions) {
        fences.insert(Segment::new(
            c.saturating_add(1),
            r,
            c.saturating_add(1),
            r.saturating_add(1),
            OutDirection::Right,
        ));
    }

    if !in_region(r.wrapping_sub(1), c, region_id, regions) {
        fences.insert(Segment::new(
            c,
            r,
            c.saturating_add(1),
            r,
            OutDirection::Up,
        ));
    }
    if !in_region(r.saturating_add(1), c, region_id, regions) {
        fences.insert(Segment::new(
            c,
            r.saturating_add(1),
            c.saturating_add(1),
            r.saturating_add(1),
            OutDirection::Down,
        ));
    }

    fences
}

/// https://adventofcode.com/2024/day/12#part2
pub fn day_12_2() {
    let data = fs::read_to_string("data/day_12.txt").expect("missing file");
    let map: Vec<_> = data
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.as_bytes())
        .collect();

    let num_rows = map.len();
    let num_cols = map[0].len();

    let regions_row = vec![EMPTY; num_cols];
    let mut regions: Vec<Vec<i32>> = vec![regions_row; num_rows];

    let mut last_region_id = -1;

    for r in 0..num_rows {
        for c in 0..num_cols {
            if regions[r][c] == EMPTY {
                let region_name = map[r][c];

                last_region_id += 1;

                let mut remaining = Vec::new();
                remaining.push((r, c));

                while let Some((r, c)) = remaining.pop() {
                    if let Some(EMPTY) =
                        regions.get(r).and_then(|row| row.get(c).copied())
                    {
                        if map[r][c] == region_name {
                            regions[r][c] = last_region_id;
                            remaining.extend(
                                [
                                    (r.wrapping_sub(1), c),
                                    (r.saturating_add(1), c),
                                    (r, c.wrapping_sub(1)),
                                    (r, c.saturating_add(1)),
                                ]
                                .iter(),
                            );
                        }
                    }
                }
            }
        }
    }

    let mut areas: HashMap<i32, usize> = HashMap::new();
    regions.iter().for_each(|row| {
        row.iter().for_each(|&plot_id| {
            *areas.entry(plot_id).or_default() += 1;
        });
    });

    let mut perimeters: HashMap<i32, PerimeterSet> = HashMap::new();
    regions.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, &plot_id)| {
            *perimeters.entry(plot_id).or_default() +=
                cell_fences(r, c, &regions);
        });
    });

    let price: usize = (0..=last_region_id)
        .map(|region_id| {
            match (areas.get(&region_id), perimeters.get(&region_id)) {
                (Some(&area), Some(perimeter)) => area * perimeter.len(),
                _ => 0,
            }
        })
        .sum();

    println!("{}", price);
}
