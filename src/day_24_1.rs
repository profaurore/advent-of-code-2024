use std::{collections::HashMap, fs};

#[derive(Eq, Hash, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Eq, Hash, PartialEq)]
struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    op: Operation,
    out: &'a str,
}

impl Gate<'_> {
    fn apply(&self, val1: bool, val2: bool) -> bool {
        match self.op {
            Operation::And => val1 & val2,
            Operation::Or => val1 | val2,
            Operation::Xor => val1 ^ val2,
        }
    }
}

/// https://adventofcode.com/2024/day/24#part1
pub fn day_24_1() {
    let data = fs::read_to_string("data/day_24.txt").expect("missing file");

    let mut lines = data.split('\n');

    let mut wire_values = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once(": ")
                .map(|(wire, value)| (wire, value.parse::<u8>().unwrap() != 0))
        })
        .collect::<HashMap<_, _>>();

    let mut gates = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts = line.splitn(5, " ").collect::<Vec<_>>();

            Gate {
                in1: parts[0],
                in2: parts[2],
                op: match parts[1] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    _ => Operation::Xor,
                },
                out: parts[4],
            }
        })
        .collect::<Vec<_>>();

    while !gates.is_empty() {
        gates.retain(|gate| {
            match (wire_values.get(gate.in1), wire_values.get(gate.in2)) {
                (Some(&val1), Some(&val2)) => {
                    wire_values.insert(gate.out, gate.apply(val1, val2));
                    false
                }

                _ => true,
            }
        });
    }

    let mut output_wires = wire_values
        .iter()
        .filter(|(&wire, _)| wire.starts_with('z'))
        .collect::<Vec<_>>();

    output_wires.sort_by(|a, b| a.0.cmp(b.0));

    let output = output_wires
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, &value))| acc + ((value as u64) << i));

    println!("{}", output);
}
