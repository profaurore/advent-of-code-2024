mod puzzle_0_not_implemented;

use std::env;
use std::{
    io::{stdin, stdout, Write},
    ops::RangeInclusive,
};

use puzzle_0_not_implemented::puzzle_0_not_implemented;

const PUZZLES: [[fn(); 2]; 25] = [
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
    [puzzle_0_not_implemented, puzzle_0_not_implemented],
];

const EXITS: [&str; 4] = ["exit", "e", "quit", "q"];

fn get_num_from_input(prompt: &str, range: RangeInclusive<usize>) -> Option<usize> {
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

fn get_num_from_str(input: &str, range: RangeInclusive<usize>) -> Option<usize> {
    match input.parse::<usize>() {
        Ok(day) if range.contains(&day) => Some(day),
        _ => None,
    }
}

fn run_puzzle(day: usize, num: usize) {
    println!("# Puzzle {} {}\n", day, num);

    PUZZLES[day - 1][num - 1]();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("ADVENT OF CODE 2025\n");

    if args.len() > 1 {
        if args.len() == 3 {
            let day = get_num_from_str(&args[1], 1..=25);
            let num = get_num_from_str(&args[2], 1..=2);

            if let (Some(day), Some(num)) = (day, num) {
                run_puzzle(day, num);
                return;
            }
        }

        println!("Expected `cargo run [<day> <num>]`\n  <day> A value between 1 and 25, inclusively.\n  <num> A value between 1 and 2, inclusively.");
        return;
    }

    loop {
        println!("(Q)uit to (e)xit.\n");

        let day = match get_num_from_input("Which day's puzzle (1-25)?", 1..=25) {
            Some(day) => day,
            None => break,
        };

        let num = match get_num_from_input("Which puzzle number (1-2)?", 1..=2) {
            Some(num) => num,
            None => break,
        };

        run_puzzle(day, num);
    }
}
