mod day_1_1;
mod day_1_2;
mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;
mod day_4_1;
mod day_4_2;
mod day_5_1;
mod day_5_2;
mod day_6_1;
mod day_6_2;
mod day_7_1;
mod day_7_2;
mod day_8_1;
mod day_8_2;
mod day_9_1;
mod day_9_2;
mod day_not_implemented;

use std::env;
use std::{
    io::{stdin, stdout, Write},
    ops::RangeInclusive,
};

use crate::{
    day_1_1::day_1_1, day_1_2::day_1_2, day_2_1::day_2_1, day_2_2::day_2_2,
    day_3_1::day_3_1, day_3_2::day_3_2, day_4_1::day_4_1, day_4_2::day_4_2,
    day_5_1::day_5_1, day_5_2::day_5_2, day_6_1::day_6_1, day_6_2::day_6_2,
    day_7_1::day_7_1, day_7_2::day_7_2, day_8_1::day_8_1, day_8_2::day_8_2,
    day_9_1::day_9_1, day_9_2::day_9_2,
    day_not_implemented::day_not_implemented,
};

const DAYS: [[fn(); 2]; 25] = [
    [day_1_1, day_1_2],
    [day_2_1, day_2_2],
    [day_3_1, day_3_2],
    [day_4_1, day_4_2],
    [day_5_1, day_5_2],
    [day_6_1, day_6_2],
    [day_7_1, day_7_2],
    [day_8_1, day_8_2],
    [day_9_1, day_9_2],
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
