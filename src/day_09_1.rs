use std::{fs, iter::repeat_n};

const EMPTY: i64 = -1;

fn position_empty_between(
    vec: &[i64],
    start: usize,
    end: usize,
) -> Option<usize> {
    if start > end {
        None
    } else {
        vec[start..end]
            .iter()
            .position(|&id| id == EMPTY)
            .map(|position| position + start)
    }
}

fn position_non_empty_between(
    vec: &[i64],
    start: usize,
    end: usize,
) -> Option<usize> {
    if start > end {
        None
    } else {
        vec[start..end]
            .iter()
            .rev()
            .position(|&id| id != EMPTY)
            .map(|position| end - position - 1)
    }
}

/// https://adventofcode.com/2024/day/9#part1
pub fn day_09_1() {
    let data = fs::read_to_string("data/day_09.txt").expect("missing file");
    let bytes = data.as_bytes();

    let mut disk: Vec<_> = bytes
        .iter()
        .take_while(|&&byte| byte != b'\n')
        .map(|&byte| byte - b'0')
        .enumerate()
        .flat_map(|(idx, count)| {
            let val = if idx % 2 == 0 { idx as i64 / 2 } else { EMPTY };

            repeat_n(val, count.into())
        })
        .collect();

    let mut idx_front_last = 0;
    let mut idx_back_last = disk.len();

    while let Some(idx_front) =
        position_empty_between(&disk, idx_front_last, idx_back_last)
    {
        if let Some(idx_back) =
            position_non_empty_between(&disk, idx_front, idx_back_last)
        {
            (disk[idx_front], disk[idx_back]) =
                (disk[idx_back], disk[idx_front]);

            idx_front_last = idx_front;
            idx_back_last = idx_back;
        } else {
            break;
        }
    }

    let checksum = disk
        .iter()
        .take_while(|&&id| id != EMPTY)
        .enumerate()
        .fold(0, |sum, (idx, &id)| sum + (idx as i64) * id);

    println!("{}", checksum);
}
