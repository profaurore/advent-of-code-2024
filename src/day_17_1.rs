use std::fs;

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
) -> impl Iterator<Item = u8> + 'a {
    lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|item| item.parse::<u8>().unwrap())
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
    fn new(register_a: u64, register_b: u64, register_c: u64) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            increment_ip: true,
            output: Vec::new(),
        }
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            x => x as u64,
        }
    }

    fn adv(&mut self, operand: u8) {
        self.register_a /= 2u64.pow(self.combo(operand) as u32);
    }

    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo(operand) % 8;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
            self.increment_ip = false;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.register_b ^= self.register_c;
    }

    fn out(&mut self, operand: u8) {
        self.output.push(self.combo(operand) % 8);
    }

    fn bdv(&mut self, operand: u8) {
        self.register_b =
            self.register_a / 2u64.pow(self.combo(operand) as u32);
    }

    fn cdv(&mut self, operand: u8) {
        self.register_c =
            self.register_a / 2u64.pow(self.combo(operand) as u32);
    }

    fn debug_registers(&self) {
        if DEBUG {
            println!(
                "Registers: {} {} {}",
                self.register_a, self.register_b, self.register_c
            );
        }
    }

    fn debug_instruction(&self, instruction: u8, operand: u8) {
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

    fn run_program(&mut self, program: &[u8]) -> &Vec<u64> {
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

/// https://adventofcode.com/2024/day/17#part1
pub fn day_17_1() {
    let data = fs::read_to_string("data/day_17.txt").expect("missing file");

    let mut lines = data.split('\n');
    let register_a = parse_register(&mut lines);
    let register_b = parse_register(&mut lines);
    let register_c = parse_register(&mut lines);
    let _blank_line = lines.next();
    let program: Vec<_> = parse_program(&mut lines).collect();

    let mut computer = Computer::new(register_a, register_b, register_c);
    let output = computer.run_program(&program);

    println!(
        "{}",
        output
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
