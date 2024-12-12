use std::fs;

const NEEDLE: &[u8] = "MAS".as_bytes();

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

fn needle_matches_at_coords_builder<'a>(
    needle: &'a [u8],
    data: &'a [u8],
    pos_from_coords: impl Fn(isize, isize) -> Option<usize> + 'a,
) -> impl Fn(usize, usize) -> bool + 'a {
    let half_needle_len: isize = (needle.len() / 2).try_into().unwrap();

    move |col: usize, row: usize| {
        let compare_fn = |col_dir: isize,
                          row_dir: isize,
                          offset: usize,
                          needle_char: &'a u8| {
            let col: isize = col.try_into().unwrap();
            let row: isize = row.try_into().unwrap();
            let offset: isize = offset.try_into().unwrap();
            let adjusted_offset = offset - half_needle_len;

            pos_from_coords(
                col + col_dir * adjusted_offset,
                row + row_dir * adjusted_offset,
            )
            .is_some_and(|pos| *needle_char == data[pos])
        };

        (needle
            .iter()
            .enumerate()
            .all(|(offset, char)| compare_fn(1, 1, offset, char))
            || needle
                .iter()
                .rev()
                .enumerate()
                .all(|(offset, char)| compare_fn(1, 1, offset, char)))
            && (needle
                .iter()
                .enumerate()
                .all(|(offset, char)| compare_fn(1, -1, offset, char))
                || needle
                    .iter()
                    .rev()
                    .enumerate()
                    .all(|(offset, char)| compare_fn(1, -1, offset, char)))
    }
}

/// https://adventofcode.com/2024/day/4#part2
pub fn day_04_2() {
    let data = fs::read_to_string("data/day_04.txt").expect("missing file");
    let data_bytes = data.as_bytes();

    let num_cols = data.find('\n');
    let mut xmas_found = 0;

    if let Some(num_cols) = num_cols {
        let num_rows = data.len() / (num_cols + 1);
        let pos_from_coords = pos_from_coords_builder(num_cols, num_rows);

        let needle_matches_at_coords = needle_matches_at_coords_builder(
            NEEDLE,
            data_bytes,
            pos_from_coords,
        );

        xmas_found = (0..num_cols).fold(0, |num_matches, col| {
            (0..num_rows).fold(num_matches, |num_matches, row| {
                num_matches + needle_matches_at_coords(col, row) as u32
            })
        });
    }

    println!("{}", xmas_found);
}
