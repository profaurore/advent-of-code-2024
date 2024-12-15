use std::fs;

const COST_A: f64 = 3.;
const COST_B: f64 = 1.;

#[derive(Debug)]
struct Offset {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Game {
    button_a: Offset,
    button_b: Offset,
    prize: Offset,
}

fn parse_line(src: &str, separator: char, plus: f64) -> Option<Offset> {
    src.split_once(':').and_then(|(_, values)| {
        values.split_once(',').and_then(|(x, y)| {
            match (
                x.split_once(separator)
                    .and_then(|val| val.1.parse::<f64>().ok()),
                y.split_once(separator)
                    .and_then(|val| val.1.parse::<f64>().ok()),
            ) {
                (Some(x), Some(y)) => Some(Offset {
                    x: x + plus,
                    y: y + plus,
                }),
                _ => None,
            }
        })
    })
}

fn parse_button(src: &str) -> Option<Offset> {
    parse_line(src, '+', 0.)
}

fn parse_prize(src: &str) -> Option<Offset> {
    parse_line(src, '=', 10_000_000_000_000.)
}

fn parse_game(
    button_a_src: &str,
    button_b_src: &str,
    prize_src: &str,
) -> Option<Game> {
    match (
        parse_button(button_a_src),
        parse_button(button_b_src),
        parse_prize(prize_src),
    ) {
        (Some(button_a), Some(button_b), Some(prize)) => Some(Game {
            button_a,
            button_b,
            prize,
        }),
        _ => None,
    }
}

/// https://adventofcode.com/2024/day/13#part2
pub fn day_13_2() {
    let data = fs::read_to_string("data/day_13.txt").expect("missing file");
    let mut lines = data.split('\n');

    let mut games: Vec<Game> = Vec::new();

    while let (Some(button_a_src), Some(button_b_src), Some(prize_src)) =
        (lines.next(), lines.next(), lines.next())
    {
        if let Some(game) = parse_game(button_a_src, button_b_src, prize_src) {
            games.push(game);
        }

        // Empty line
        lines.next();
    }

    // a * button_a.x + b * button_b.x = prize.x
    // a * button_a.y + b * button_b.y = prize.y
    // a * a_cost + b * b_cost = tokens
    // a + b <= 100
    let total_tokens = games.iter().fold(0f64, |sum, game| {
        let Game {
            button_a,
            button_b,
            prize,
        } = game;
        let button_a_ratio = button_a.x / button_a.y;
        let button_b_ratio = button_b.x / button_b.y;
        let prize_ratio = prize.x / prize.y;

        // Simple cases (no or single solution)
        if button_a_ratio == button_b_ratio && button_a_ratio == prize_ratio {
            return sum;
        }

        let b = (button_a.x * prize.y - button_a.y * prize.x)
            / (button_a.x * button_b.y - button_a.y * button_b.x);
        let a = (prize.x - b * button_b.x) / button_a.x;

        let tokens = if b == b.round() {
            a * COST_A + b * COST_B
        } else {
            0.
        };

        sum + tokens
    }) as u64;

    println!("{:?}", total_tokens);
}
