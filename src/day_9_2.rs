use std::{fs, iter::repeat_n};

const EMPTY: i64 = -1;

fn position_file_id_before(
    vec: &[(i64, u8)],
    file_id: i64,
    end: usize,
) -> Option<usize> {
    if file_id < 0 {
        None
    } else {
        vec[0..end]
            .iter()
            .rev()
            .position(|&(id, _)| id == file_id)
            .map(|position| end - position - 1)
    }
}

fn position_empty_before_min(
    vec: &[(i64, u8)],
    end: usize,
    min_size: u8,
) -> Option<usize> {
    vec[0..end]
        .iter()
        .position(|&(id, size)| id == EMPTY && size >= min_size)
}

/// https://adventofcode.com/2024/day/9#part2
pub fn day_9_2() {
    let data = fs::read_to_string("data/day_9.txt").expect("missing file");
    let bytes = data.as_bytes();

    let mut disk_files: Vec<_> = bytes
        .iter()
        .take_while(|&&byte| byte != b'\n')
        .map(|&byte| byte - b'0')
        .enumerate()
        .map(|(idx, size)| {
            let val = if idx % 2 == 0 { idx as i64 / 2 } else { EMPTY };

            (val, size)
        })
        .filter(|&(_, size)| size > 0)
        .collect();

    let mut file_id = disk_files.last().map_or(-1, |&(id, _)| id);
    let mut idx_file_prev = disk_files.len();

    while let Some(idx_file) =
        position_file_id_before(&disk_files, file_id, idx_file_prev)
    {
        let file = disk_files[idx_file];

        if let Some(idx_empty) =
            position_empty_before_min(&disk_files, idx_file, file.1)
        {
            // Update the size of the empty space.
            disk_files[idx_empty].1 -= file.1;

            // Update the previous location of the empty file to be empty.
            // We don't need to merge the empty spaces between the files are
            // always moving forward.
            disk_files[idx_file].0 = EMPTY;

            // Insert the moved file.
            disk_files.insert(idx_empty, file);

            // Account for the inserted file.
            idx_file_prev = idx_file + 1;
        } else {
            idx_file_prev = idx_file;
        }

        file_id -= 1;
    }

    let disk: Vec<_> = disk_files
        .iter()
        .flat_map(|&(id, size)| repeat_n(id, size.into()))
        .collect();

    let checksum = disk
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != EMPTY)
        .fold(0, |sum, (idx, &id)| sum + (idx as i64) * id);

    println!("{}", checksum);
}
