use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::repeat_n,
};

const DEBUG: bool = false;

struct Code<'a> {
    num: usize,
    buttons: &'a [u8],
}

type KeyDef = ((usize, usize), u8);

const NUMPAD_KEYS: [KeyDef; 11] = [
    ((2, 3), b'A'),
    ((1, 3), b'0'),
    ((0, 2), b'1'),
    ((1, 2), b'2'),
    ((2, 2), b'3'),
    ((0, 1), b'4'),
    ((1, 1), b'5'),
    ((2, 1), b'6'),
    ((0, 0), b'7'),
    ((1, 0), b'8'),
    ((2, 0), b'9'),
];

const DIRPAD_KEYS: [KeyDef; 5] = [
    ((2, 0), b'A'),
    ((1, 0), b'^'),
    ((0, 1), b'<'),
    ((1, 1), b'v'),
    ((2, 1), b'>'),
];

fn format_ascii(seq: &[u8]) -> String {
    String::from_utf8(seq.to_vec()).unwrap()
}

fn gen_permutations(items: &[u8]) -> HashSet<Vec<u8>> {
    let mut permutations: HashSet<Vec<_>> = HashSet::new();

    let size = items.len();

    (0..size.pow(size as u32))
        .map(|idx| {
            let mut remaining = idx;

            (0..size)
                .map(|_| {
                    let idx = remaining % size;
                    remaining /= size;
                    idx
                })
                .collect::<Vec<_>>()
        })
        .filter(|indices| {
            indices.iter().collect::<HashSet<_>>().len() == indices.len()
        })
        .for_each(|indices| {
            permutations.insert(
                indices
                    .iter()
                    .map(|&idx| items[idx])
                    .chain([b'A'].iter().copied())
                    .collect(),
            );
        });

    permutations
}

fn validate_permutations(
    x: usize,
    y: usize,
    permutations: &mut HashSet<Vec<u8>>,
    keys: &[KeyDef],
) {
    let keys_hash = keys.iter().copied().collect::<HashMap<_, _>>();

    permutations.retain(|moves| {
        let mut last_pos = (x, y);

        moves.iter().filter(|&&m| m != b'A').all(|&m| {
            let new_pos = match m {
                b'<' => (last_pos.0 - 1, last_pos.1),
                b'^' => (last_pos.0, last_pos.1 - 1),
                b'>' => (last_pos.0 + 1, last_pos.1),
                b'v' => (last_pos.0, last_pos.1 + 1),
                _ => last_pos,
            };

            let is_valid = keys_hash.contains_key(&new_pos);

            last_pos = new_pos;

            is_valid
        })
    });
}

struct Pad<'a> {
    name: &'a str,
    move_sets: HashMap<(u8, u8), HashSet<Vec<u8>>>,
    subpad: Option<Box<Pad<'a>>>,
}

impl<'a> Pad<'a> {
    fn new(name: &'a str, keys: &[KeyDef]) -> Self {
        let move_sets = keys
            .iter()
            .flat_map(|&((x1, y1), key1)| {
                keys.iter().map(move |&((x2, y2), key2)| {
                    let dx = x2.abs_diff(x1);
                    let dy = y2.abs_diff(y1);

                    let horizontal_move = if x2 > x1 { b'>' } else { b'<' };
                    let vertical_move = if y2 > y1 { b'v' } else { b'^' };

                    let moves = Vec::from_iter(
                        repeat_n(horizontal_move, dx)
                            .chain(repeat_n(vertical_move, dy)),
                    );

                    let mut permutations = gen_permutations(&moves);

                    validate_permutations(x1, y1, &mut permutations, keys);

                    ((key1, key2), permutations)
                })
            })
            .collect();

        Self {
            name,
            move_sets,
            subpad: None,
        }
    }

    fn get_sequence_min(&self, seq: &[u8]) -> Option<usize> {
        if DEBUG {
            println!("start {} {}", self.name, format_ascii(seq));
        }

        let min = seq
            .iter()
            .try_fold((0, b'A'), |(len, prev), &button| {
                self.get_min_dist(prev, button)
                    .map(|min| (len + min, button))
            })
            .map(|(len, _)| len);

        if DEBUG {
            println!(
                "end {} {:?} {}",
                self.name,
                min,
                String::from_utf8(seq.to_vec()).unwrap()
            );
        }

        min
    }

    fn get_min_dist(&self, k1: u8, k2: u8) -> Option<usize> {
        if DEBUG {
            println!(
                "  start {} {} {} {:?}",
                self.name,
                k1 as char,
                k2 as char,
                self.move_sets.get(&(k1, k2))
            );
        }

        let min = self.move_sets.get(&(k1, k2)).and_then(|move_set| {
            move_set
                .iter()
                .filter_map(|moves| match &self.subpad {
                    Some(subpad) => subpad.get_sequence_min(moves),
                    _ => Some(moves.len()),
                })
                .min()
        });

        if DEBUG {
            println!(
                "  end {} {} {} {:?} {:?}",
                self.name,
                k1 as char,
                k2 as char,
                min,
                self.move_sets.get(&(k1, k2))
            );
        }

        min
    }

    fn chain(self, name: &'a str, keys: &[KeyDef]) -> Pad<'a> {
        let mut pad = Self::new(name, keys);
        pad.subpad = Some(Box::new(self));

        pad
    }
}

/// https://adventofcode.com/2024/day/21#part1
pub fn day_21_1() {
    let data = fs::read_to_string("data/day_21.txt").expect("missing file");

    let codes = data
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|line| Code {
            num: line[..line.len() - 1].parse::<usize>().unwrap(),
            buttons: line.as_bytes(),
        })
        .collect::<Vec<_>>();

    let numpad = Pad::new("Dirpad-2", &DIRPAD_KEYS)
        .chain("Dirpad-1", &DIRPAD_KEYS)
        .chain("Numpad", &NUMPAD_KEYS);

    let complexities_sum = codes
        .iter()
        .map(|Code { num, buttons }| {
            let shortest_sequence = numpad.get_sequence_min(buttons).unwrap();

            shortest_sequence * num
        })
        .sum::<usize>();

    println!("{}", complexities_sum);
}
