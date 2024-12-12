use std::{collections::HashSet, fs};

fn find_trail_ends(
    data: &[u8],
    trail_offsets: &[isize; 4],
    position: usize,
    next_height: u8,
) -> HashSet<usize> {
    if next_height > 9 {
        HashSet::from_iter([position])
    } else {
        trail_offsets
            .iter()
            .filter_map(|&offset| {
                position
                    .checked_add_signed(offset)
                    .and_then(|next_position| {
                        data.get(next_position).and_then(|&height| {
                            if height != b'\n' && height - b'0' == next_height {
                                Some(find_trail_ends(
                                    data,
                                    trail_offsets,
                                    next_position,
                                    next_height + 1,
                                ))
                            } else {
                                None
                            }
                        })
                    })
            })
            .flatten()
            .collect()
    }
}

/// https://adventofcode.com/2024/day/10#part1
pub fn day_10_1() {
    let data = fs::read_to_string("data/day_10.txt").expect("missing file");
    let bytes = data.as_bytes();

    let mut scores_sum: usize = 0;

    if let Some(num_cols) = data.find('\n') {
        let row_offset = 0isize.saturating_add_unsigned(num_cols + 1);
        let trail_offsets = [-row_offset, 1, row_offset, -1];

        let trailheads = bytes
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, height)| height == b'0')
            .map(|(position, _)| position);

        scores_sum = trailheads
            .map(|position| {
                find_trail_ends(bytes, &trail_offsets, position, 1).len()
            })
            .sum();
    }

    println!("{}", scores_sum);
}
