use std::fs;

const NEEDLE: &[u8] = "XMAS".as_bytes();

fn pos_from_coords_builder(
    num_cols: usize,
    num_rows: usize,
) -> impl Fn(isize, isize) -> Option<usize> {
    let row_offset = num_rows + 1;

    move |col, row| {
        let num_cols: isize = num_cols.try_into().unwrap();
        let num_rows: isize = num_rows.try_into().unwrap();

        if (0..num_cols).contains(&col) && (0..num_rows).contains(&row) {
            Some(row.unsigned_abs() * row_offset + col.unsigned_abs())
        } else {
            None
        }
    }
}

fn needle_matches_for_dir_builder<'a>(
    needle: &'a [u8],
    data: &'a [u8],
    num_cols: usize,
    num_rows: usize,
    pos_from_coords: impl Fn(isize, isize) -> Option<usize> + 'a,
) -> impl Fn(isize, isize) -> u32 + 'a {
    move |col_dir: isize, row_dir: isize| {
        let compare_fn =
            |col: usize, row: usize, offset: usize, needle_char: &'a u8| {
                let col: isize = col.try_into().unwrap();
                let row: isize = row.try_into().unwrap();
                let offset: isize = offset.try_into().unwrap();

                pos_from_coords(col + col_dir * offset, row + row_dir * offset)
                    .is_some_and(|pos| *needle_char == data[pos])
            };

        (0..num_cols).fold(0, |num_matches, col| {
            (0..num_rows).fold(num_matches, |num_matches, row| {
                num_matches
                    + needle.iter().enumerate().all(|(offset, char)| {
                        compare_fn(col, row, offset, char)
                    }) as u32
                    + needle.iter().rev().enumerate().all(|(offset, char)| {
                        compare_fn(col, row, offset, char)
                    }) as u32
            })
        })
    }
}

/// https://adventofcode.com/2024/day/4#part1
pub fn day_4_1() {
    let data = fs::read_to_string("data/day_4.txt").expect("missing file");
    let data_bytes = data.as_bytes();

    let num_cols = data.find('\n');
    let mut xmas_found = 0;

    if let Some(num_cols) = num_cols {
        let num_rows = data.len() / (num_cols + 1);
        let pos_from_coords = pos_from_coords_builder(num_cols, num_rows);

        let needle_matches_for_dir = needle_matches_for_dir_builder(
            NEEDLE,
            data_bytes,
            num_cols,
            num_rows,
            pos_from_coords,
        );

        // Right & left matches
        xmas_found += needle_matches_for_dir(1, 0);

        // Down & up matches
        xmas_found += needle_matches_for_dir(0, 1);

        // Right-down & left-up matches
        xmas_found += needle_matches_for_dir(1, 1);

        // Right-up & left-down matches
        xmas_found += needle_matches_for_dir(1, -1);
    }

    println!("{}", xmas_found);
}
