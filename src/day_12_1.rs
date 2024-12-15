use std::{collections::HashMap, fs};

const EMPTY: i32 = -1;

fn in_region(r: usize, c: usize, region_id: i32, regions: &[Vec<i32>]) -> bool {
    regions
        .get(r)
        .and_then(|row| row.get(c))
        .is_some_and(|&cell_id| cell_id == region_id)
}

fn cell_fences(r: usize, c: usize, regions: &[Vec<i32>]) -> usize {
    let region_id = regions[r][c];

    !in_region(r.wrapping_sub(1), c, region_id, regions) as usize
        + !in_region(r.saturating_add(1), c, region_id, regions) as usize
        + !in_region(r, c.wrapping_sub(1), region_id, regions) as usize
        + !in_region(r, c.saturating_add(1), region_id, regions) as usize
}

/// https://adventofcode.com/2024/day/12#part1
pub fn day_12_1() {
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

    let mut perimeters: HashMap<i32, usize> = HashMap::new();
    regions.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, &plot_id)| {
            *perimeters.entry(plot_id).or_default() +=
                cell_fences(r, c, &regions);
        });
    });

    let price: usize = (0..=last_region_id)
        .map(|region_id| {
            areas.get(&region_id).copied().unwrap_or_default()
                * perimeters.get(&region_id).copied().unwrap_or_default()
        })
        .sum();

    println!("{}", price);
}
