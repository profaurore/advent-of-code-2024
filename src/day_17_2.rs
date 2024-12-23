use std::{fs, ops::Range};

const DEBUG: bool = false;

fn parse_register<'a>(lines: &mut impl Iterator<Item = &'a str>) -> u64 {
    lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<u64>()
        .unwrap()
}

fn parse_program<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = u64> + 'a {
    lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|item| item.parse::<u64>().unwrap())
}

struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    increment_ip: bool,
    output: Vec<u64>,
}

impl Computer {
    fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            increment_ip: true,
            output: Vec::new(),
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            x => x,
        }
    }

    fn adv(&mut self, operand: u64) {
        // Equivalent to a / 2 ** o
        self.register_a >>= self.combo(operand);
    }

    fn bxl(&mut self, operand: u64) {
        self.register_b ^= operand;
    }

    fn bst(&mut self, operand: u64) {
        // Equivalent to o % 8
        self.register_b = self.combo(operand) & 7;
    }

    fn jnz(&mut self, operand: u64) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
            self.increment_ip = false;
        }
    }

    fn bxc(&mut self, _operand: u64) {
        self.register_b ^= self.register_c;
    }

    fn out(&mut self, operand: u64) {
        // Equivalent to o % 8
        self.output.push(self.combo(operand) & 7);
    }

    fn bdv(&mut self, operand: u64) {
        // Equivalent to a / 2 ** o
        self.register_b = self.register_a >> self.combo(operand);
    }

    fn cdv(&mut self, operand: u64) {
        // Equivalent to a / 2 ** o
        self.register_c = self.register_a >> self.combo(operand);
    }

    fn debug_registers(&self) {
        if DEBUG {
            println!(
                "Registers: {} {} {}",
                self.register_a, self.register_b, self.register_c
            );
        }
    }

    fn debug_instruction(&self, instruction: u64, operand: u64) {
        if DEBUG {
            println!(
                "Instruction: {} {} {}",
                self.instruction_pointer,
                match instruction {
                    0 => "adv",
                    1 => "blx",
                    2 => "bst",
                    3 => "jnz",
                    4 => "bxc",
                    5 => "out",
                    6 => "bdv",
                    7 => "cdv",
                    _ => "unknown",
                },
                operand
            );
        }
    }

    fn run_program(
        &mut self,
        register_a: u64,
        register_b: u64,
        register_c: u64,
        program: &[u64],
    ) -> &Vec<u64> {
        self.register_a = register_a;
        self.register_b = register_b;
        self.register_c = register_c;
        self.instruction_pointer = 0;
        self.increment_ip = true;
        self.output.clear();

        self.debug_registers();

        while let (Some(&instruction), Some(&operand)) = (
            program.get(self.instruction_pointer),
            program.get(self.instruction_pointer + 1),
        ) {
            self.debug_instruction(instruction, operand);

            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => (),
            }

            self.debug_registers();

            if self.increment_ip {
                self.instruction_pointer += 2;
            }
            self.increment_ip = true;
        }

        &self.output
    }
}

fn search_for_output(
    computer: &mut Computer,
    register_a_range: Range<u64>,
    register_b: u64,
    register_c: u64,
    program: &[u64],
    idx: usize,
) -> Vec<Range<u64>> {
    let value = program[idx];
    let values = &program[idx..];

    let step = (register_a_range.clone().count() / 32).max(1);
    let outputs = register_a_range
        .clone()
        .step_by(step)
        .chain([register_a_range.end - 1].iter().copied())
        .map(|register_a| {
            (
                register_a,
                computer
                    .run_program(register_a, register_b, register_c, program)
                    [idx],
            )
        })
        .collect::<Vec<_>>();

    let mut search_idx = 0;
    let mut ranges = Vec::new();

    while let Some(range_start_pos) = outputs
        .iter()
        .skip(search_idx)
        .position(|&(_, output)| output == value)
        .map(|pos| pos + search_idx)
    {
        let mut range_start = outputs[range_start_pos].0;
        if let Some((mut before_range, _)) =
            outputs.get(range_start_pos.wrapping_sub(1))
        {
            while before_range < range_start {
                let mid = range_start.midpoint(before_range);

                let mid_output = &computer
                    .run_program(mid, register_b, register_c, program)[idx..];

                if mid_output == values {
                    range_start = mid;
                } else {
                    before_range = mid + 1;
                }
            }
        }

        let range_end_pos = outputs
            .iter()
            .skip(range_start_pos)
            .position(|&(_, output)| output != value)
            .map_or_else(|| outputs.len() - 1, |pos| range_start_pos + pos - 1);
        let mut range_end = outputs[range_end_pos].0;
        if let Some((mut after_range, _)) = outputs.get(range_end_pos + 1) {
            while range_end < after_range {
                let mid = range_end.midpoint(after_range) + 1;

                let mid_output = &computer
                    .run_program(mid, register_b, register_c, program)[idx..];

                if mid_output == values {
                    range_end = mid;
                } else {
                    after_range = mid - 1;
                }
            }
        }

        search_idx = range_end_pos + 1;
        ranges.push(range_start..(range_end + 1));
    }

    ranges
}

/// https://adventofcode.com/2024/day/17#part2
pub fn day_17_2() {
    let data = fs::read_to_string("data/day_17.txt").expect("missing file");

    let mut lines = data.split('\n');
    let _register_a = parse_register(&mut lines);
    let register_b = parse_register(&mut lines);
    let register_c = parse_register(&mut lines);
    let _blank_line = lines.next();
    let program: Vec<_> = parse_program(&mut lines).collect();

    let mut computer = Computer::new();

    let program_len = program.len();

    // Value of `register_a` where the size of the output of the program matches
    // the size of the program.
    #[allow(clippy::single_range_in_vec_init)]
    let mut ranges: Vec<Range<u64>> =
        vec![(8u64.pow(program_len as u32 - 1)..8u64.pow(program_len as u32))];

    // This uses a rediculous amount of knowledge about the input program and
    // does not generalize to any other programs.
    for i in (0..program_len).rev() {
        ranges = ranges
            .iter()
            .flat_map(|range| {
                search_for_output(
                    &mut computer,
                    range.clone(),
                    register_b,
                    register_c,
                    &program,
                    i,
                )
            })
            .collect();
    }

    let register_a = ranges
        .iter()
        .find_map(|range| {
            range.clone().find(|&register_a| {
                let output = computer
                    .run_program(register_a, register_b, register_c, &program);
                output == &program
            })
        })
        .unwrap();

    println!("{}", register_a);
}
