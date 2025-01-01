use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    hash::Hash,
};

const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    op: Operation,
    out: &'a str,
}

impl<'a> Gate<'a> {
    fn new(in1: &'a str, op: Operation, in2: &'a str, out: &'a str) -> Self {
        let (min, max) = if in1 < in2 { (in1, in2) } else { (in2, in1) };

        Self {
            in1: min,
            in2: max,
            op,
            out,
        }
    }

    fn apply(&self, val1: bool, val2: bool) -> bool {
        match self.op {
            Operation::And => val1 & val2,
            Operation::Or => val1 | val2,
            Operation::Xor => val1 ^ val2,
        }
    }
}

#[derive(Clone)]
struct Circuit<'a> {
    wire_values: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
    num_input_bits: usize,
    num_output_bits: usize,
    output: u64,
}

impl<'a> Circuit<'a> {
    fn new(wire_values: HashMap<&'a str, bool>, gates: Vec<Gate<'a>>) -> Self {
        let num_input_bits = wire_values
            .iter()
            .filter(|(&wire, _)| wire.starts_with('x'))
            .count();
        let num_output_bits = gates
            .iter()
            .filter(|gate| gate.out.starts_with('z'))
            .count();

        Self {
            wire_values,
            gates,
            num_input_bits,
            num_output_bits,
            output: 0,
        }
    }

    fn wires_to_values(&self, set: char) -> u64 {
        self.wire_values
            .iter()
            .filter(|(&wire, _)| wire.starts_with(set))
            .fold(0, |acc, (&wire, &value)| {
                if value {
                    acc + (1 << wire[1..].parse::<u8>().unwrap())
                } else {
                    acc
                }
            })
    }

    fn bad_bits(&self) -> u64 {
        self.expected_output() ^ self.output
    }

    fn is_bits_ok(&self) -> bool {
        self.bad_bits() == 0
    }

    fn set_wire_value(&mut self, set: char, new_value: u64) {
        self.wire_values
            .iter_mut()
            .filter(|(&wire, _)| wire.starts_with(set))
            .for_each(|(&wire, value)| {
                *value =
                    new_value & (1 << wire[1..].parse::<u8>().unwrap()) != 0;
            });
    }

    fn expected_output(&self) -> u64 {
        self.wires_to_values('x') + self.wires_to_values('y')
    }

    // WARNING: Do NOT run twice in a row. Wire values are not reset.
    fn run(&mut self) -> bool {
        let mut remaining_gates = self.gates.clone();

        while !remaining_gates.is_empty() {
            let num_remaining_gates = remaining_gates.len();

            remaining_gates.retain(|gate| {
                match (
                    self.wire_values.get(gate.in1),
                    self.wire_values.get(gate.in2),
                ) {
                    (Some(&val1), Some(&val2)) => {
                        self.wire_values
                            .insert(gate.out, gate.apply(val1, val2));
                        false
                    }

                    _ => true,
                }
            });

            // Infinite loop
            if remaining_gates.len() == num_remaining_gates {
                return false;
            }
        }

        self.output = self.wires_to_values('z');

        true
    }

    fn run_with_values(&mut self, x: u64, y: u64) -> bool {
        self.set_wire_value('x', x);
        self.set_wire_value('y', y);
        self.run()
    }

    fn print_gates(&self, wire: Option<&str>) {
        let mut seen = HashSet::new();

        let mut remaining = self
            .gates
            .iter()
            .filter(|gate| match wire {
                Some(wire) => gate.out == wire,
                _ => gate.out.starts_with('z'),
            })
            .map(|gate| (gate.out, 0))
            .collect::<Vec<_>>();
        remaining.sort_by(|a, b| b.0.cmp(a.0));

        while let Some((wire, depth)) = remaining.pop() {
            if seen.contains(&wire) {
                println!("{:depth$}{wire}  (seen)", "",);
            } else if let Some(gate) =
                self.gates.iter().find(|gate| gate.out == wire)
            {
                println!(
                    "{:depth$}{wire}  {} {:?} {}",
                    "", gate.in1, gate.op, gate.in2
                );
                remaining.push((gate.in2, depth + 1));
                remaining.push((gate.in1, depth + 1));

                seen.insert(wire);
            }
        }

        println!();
    }

    fn swap_gates(&mut self, gate_a: &str, gate_b: &str) {
        let gate_idx_a = self.gates.iter().position(|g| g.out == gate_a);
        let gate_idx_b = self.gates.iter().position(|g| g.out == gate_b);

        if let (Some(gate_idx_a), Some(gate_idx_b)) = (gate_idx_a, gate_idx_b) {
            (self.gates[gate_idx_a].out, self.gates[gate_idx_b].out) =
                (self.gates[gate_idx_b].out, self.gates[gate_idx_a].out);
        }
    }
}

impl Debug for Circuit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "in: {}, out: {}\nin1:  {:#0in_width$b}\nin2:  {:#0in_width$b}\nout: {:#0out_width$b}\nact: {:#0out_width$b}\ndif: {:#0out_width$b}",
            self.num_input_bits,
            self.num_output_bits,
            self.wires_to_values('x'),
            self.wires_to_values('y'),
            self.expected_output(),
            self.output,
            self.bad_bits(),
            // +2 to account for "0b"
            in_width = self.num_input_bits + 2,
            out_width = self.num_output_bits + 2
        )
    }
}

fn get_pairs_from_set(list: Vec<&str>) -> Vec<Vec<(&str, &str)>> {
    if list.is_empty() {
        return vec![Vec::new()];
    }

    list[1..]
        .iter()
        .enumerate()
        .flat_map(|(idx, item)| {
            let mut remaining = list.clone();
            remaining.remove(0);
            remaining.remove(idx);

            let mut result = get_pairs_from_set(remaining.clone());
            result.iter_mut().for_each(|l| l.push((list[0], item)));

            result
        })
        .collect()
}

/// https://adventofcode.com/2024/day/24#part2
pub fn day_24_2() {
    let data = fs::read_to_string("data/day_24.txt").expect("missing file");

    let mut lines = data.split('\n');

    let wire_values = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once(": ")
                .map(|(wire, value)| (wire, value.parse::<u8>().unwrap() != 0))
        })
        .collect::<HashMap<_, _>>();

    let gates = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts = line.splitn(5, " ").collect::<Vec<_>>();

            Gate::new(
                parts[0],
                match parts[1] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    _ => Operation::Xor,
                },
                parts[2],
                parts[4],
            )
        })
        .collect::<Vec<_>>();

    let circuit = Circuit::new(wire_values, gates);

    if DEBUG {
        circuit.print_gates(None);
    }

    // Bad output values.
    let bad_z_gates = circuit
        .gates
        .iter()
        .filter(|gate| {
            let out = gate.out;

            out.starts_with('z')
                && gate.op != Operation::Xor
                && out[1..].parse::<usize>().unwrap()
                    < circuit.num_output_bits - 1
        })
        .map(|gate| gate.out)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("bad z gates {:?}\n", bad_z_gates);
    }

    let bad_l2_gates = circuit
        .gates
        .iter()
        .filter(|&gate| {
            gate.out.starts_with('z')
                && !bad_z_gates.contains(&gate.out)
                && gate.out[1..]
                    .parse::<usize>()
                    .is_ok_and(|num| (2..circuit.num_input_bits).contains(&num))
        })
        .flat_map(|gate| {
            let gate_num = &gate.out[1..];

            let in1 = circuit.gates.iter().find(|g| g.out == gate.in1);
            let in2 = circuit.gates.iter().find(|g| g.out == gate.in2);

            let in1_is_or = in1.is_some_and(|g| g.op == Operation::Or);
            let in2_is_or = in2.is_some_and(|g| g.op == Operation::Or);

            let in1_is_input_xor = in1.is_some_and(|g| {
                g.op == Operation::Xor
                    && g.in1.starts_with('x')
                    && &g.in1[1..] == gate_num
                    && g.in2.starts_with('y')
                    && &g.in2[1..] == gate_num
            });
            let in2_is_input_xor = in2.is_some_and(|g| {
                g.op == Operation::Xor
                    && g.in1.starts_with('x')
                    && &g.in1[1..] == gate_num
                    && g.in2.starts_with('y')
                    && &g.in2[1..] == gate_num
            });

            let or_gate_name = if in1_is_input_xor {
                if !in2_is_or {
                    if DEBUG {
                        println!("{} {}", gate.out, gate.in2);
                    }

                    return vec![gate.in2];
                }

                gate.in2
            } else if in2_is_input_xor {
                if !in1_is_or {
                    if DEBUG {
                        println!("{} {}", gate.out, gate.in1);
                    }

                    return vec![gate.in1];
                }

                gate.in1
            } else if in1_is_or {
                if DEBUG {
                    println!("{} {}", gate.out, gate.in2);
                }

                return vec![gate.in2];
            } else if in2_is_or {
                if DEBUG {
                    println!("{} {}", gate.out, gate.in1);
                }

                return vec![gate.in1];
            } else {
                return vec![];
            };

            let or_gate = circuit
                .gates
                .iter()
                .find(|g| g.out == or_gate_name)
                .unwrap();

            let in1 = circuit.gates.iter().find(|g| g.out == or_gate.in1);
            let in2 = circuit.gates.iter().find(|g| g.out == or_gate.in2);

            let in1_is_input_and = in1.is_some_and(|g| g.op == Operation::And);
            let in2_is_input_and = in2.is_some_and(|g| g.op == Operation::And);

            if !in1_is_input_and && !in2_is_input_and {
                if DEBUG {
                    println!(
                        "{}-{} {} {}",
                        gate.out, or_gate_name, or_gate.in1, or_gate.in2
                    );
                }

                return vec![or_gate.in1, or_gate.in2];
            } else if !in1_is_input_and {
                if DEBUG {
                    println!("{}-{} {}", gate.out, or_gate_name, or_gate.in1);
                }

                return vec![or_gate.in1];
            } else if !in2_is_input_and {
                if DEBUG {
                    println!("{}-{} {}", gate.out, or_gate_name, or_gate.in2);
                }

                return vec![or_gate.in2];
            }

            vec![]
        })
        .collect::<Vec<_>>();

    if DEBUG {
        println!("bad l2 gates {:?}\n", bad_l2_gates);
    }

    let mut bad_gates = [bad_z_gates, bad_l2_gates].concat();
    bad_gates.sort();

    if DEBUG {
        let possible_pair_sets = get_pairs_from_set(bad_gates.clone());

        possible_pair_sets.iter().for_each(|pair_set| {
            let alternating_bits = (0..circuit.num_input_bits)
                .step_by(2)
                .fold(0, |acc, i| acc + (1 << i));

            let passed = [
                ((1 << circuit.num_input_bits) - 1, 1),
                ((1 << (circuit.num_input_bits / 2)) - 1, 1),
                (
                    ((1 << (circuit.num_input_bits / 2)) - 1)
                        << (circuit.num_input_bits.div_ceil(2)),
                    1,
                ),
                (alternating_bits, 1),
                (alternating_bits >> 1, 1),
            ]
            .iter()
            .all(|&(x, y)| {
                let mut swap_test = circuit.clone();
                pair_set.iter().for_each(|pair| {
                    swap_test.swap_gates(pair.0, pair.1);
                });

                swap_test.run_with_values(x, y);

                swap_test.is_bits_ok()
            });

            if passed {
                println!("valid pairs {:?}\n", pair_set);
            }
        });
    }

    println!("{}", bad_gates.join(","));
}
