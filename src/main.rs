mod day_01_1;
mod day_01_2;
mod day_02_1;
mod day_02_2;
mod day_03_1;
mod day_03_2;
mod day_04_1;
mod day_04_2;
mod day_05_1;
mod day_05_2;
mod day_06_1;
mod day_06_2;
mod day_07_1;
mod day_07_2;
mod day_08_1;
mod day_08_2;
mod day_09_1;
mod day_09_2;
mod day_10_1;
mod day_10_2;
mod day_11_1;
mod day_11_2;
mod day_12_1;
mod day_12_2;
mod day_13_1;
mod day_13_2;
mod day_14_1;
mod day_14_2;
mod day_not_implemented;

use std::env;
use std::{
    io::{stdin, stdout, Write},
    ops::RangeInclusive,
};

use crate::{
    day_01_1::day_01_1, day_01_2::day_01_2, day_02_1::day_02_1,
    day_02_2::day_02_2, day_03_1::day_03_1, day_03_2::day_03_2,
    day_04_1::day_04_1, day_04_2::day_04_2, day_05_1::day_05_1,
    day_05_2::day_05_2, day_06_1::day_06_1, day_06_2::day_06_2,
    day_07_1::day_07_1, day_07_2::day_07_2, day_08_1::day_08_1,
    day_08_2::day_08_2, day_09_1::day_09_1, day_09_2::day_09_2,
    day_10_1::day_10_1, day_10_2::day_10_2, day_11_1::day_11_1,
    day_11_2::day_11_2, day_12_1::day_12_1, day_12_2::day_12_2,
    day_13_1::day_13_1, day_13_2::day_13_2, day_14_1::day_14_1,
    day_14_2::day_14_2, day_not_implemented::day_not_implemented,
};

const DAYS: [[fn(); 2]; 25] = [
    [day_01_1, day_01_2],
    [day_02_1, day_02_2],
    [day_03_1, day_03_2],
    [day_04_1, day_04_2],
    [day_05_1, day_05_2],
    [day_06_1, day_06_2],
    [day_07_1, day_07_2],
    [day_08_1, day_08_2],
    [day_09_1, day_09_2],
    [day_10_1, day_10_2],
    [day_11_1, day_11_2],
    [day_12_1, day_12_2],
    [day_13_1, day_13_2],
    [day_14_1, day_14_2],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
    [day_not_implemented, day_not_implemented],
];

const EXITS: [&str; 4] = ["exit", "e", "quit", "q"];

fn get_num_from_input(
    prompt: &str,
    range: RangeInclusive<usize>,
) -> Option<usize> {
    loop {
        println!("{}", prompt);
        let _ = stdout().flush();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_ok() {
            input = input.trim().to_lowercase();

            if EXITS.iter().any(|exit| *exit == input) {
                return None;
            }

            if let Ok(day) = input.parse::<usize>() {
                if range.contains(&day) {
                    println!();
                    return Some(day);
                }
            }
        }

        println!(
            "Expected a number between {} and {}!\n",
            range.start(),
            range.end()
        );
    }
}

fn get_num_from_str(
    input: &str,
    range: RangeInclusive<usize>,
) -> Option<usize> {
    match input.parse::<usize>() {
        Ok(day) if range.contains(&day) => Some(day),
        _ => None,
    }
}

fn run_day(day: usize, part: usize) {
    println!("# Day {} №{}\n", day, part);

    DAYS[day - 1][part - 1]();

    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("ADVENT OF CODE 2025\n");

    if args.len() > 1 {
        if args.len() == 3 {
            let day = get_num_from_str(&args[1], 1..=25);
            let num = get_num_from_str(&args[2], 1..=2);

            if let (Some(day), Some(num)) = (day, num) {
                run_day(day, num);
                return;
            }
        }

        println!("Expected `cargo run [<day> <num>]`\n  <day> A value between 1 and 25, inclusively.\n  <num> A value between 1 and 2, inclusively.");
        return;
    }

    loop {
        println!("(Q)uit to (e)xit.\n");

        let day = match get_num_from_input("Which day (1-25)?", 1..=25) {
            Some(day) => day,
            None => break,
        };

        let num = match get_num_from_input("Which part (1-2)?", 1..=2) {
            Some(num) => num,
            None => break,
        };

        run_day(day, num);
    }
}
