use std::fs;

/// https://adventofcode.com/2024/day/2#part1
pub fn day_02_1() {
    let data = fs::read_to_string("data/day_02.txt").expect("missing file");

    let num_safe = data.split('\n').filter(|line| !line.is_empty()).fold(
        0,
        |num_safe, report| {
            let levels: Vec<_> = report
                .split(" ")
                .filter_map(|level| level.parse::<i8>().ok())
                .collect();

            let (is_gradual, directions_sum) =
                levels.windows(2).flat_map(<&[i8; 2]>::try_from).fold(
                    (true, 0),
                    |(is_gradual, directions_sum), &[level_a, level_b]| {
                        (
                            is_gradual
                                && (1..=3).contains(&level_b.abs_diff(level_a)),
                            directions_sum + (level_b - level_a).signum(),
                        )
                    },
                );

            let abs_directions_sum: usize = directions_sum.abs_diff(0).into();
            let is_strictly_monotonic =
                abs_directions_sum == levels.len().saturating_sub(1);

            num_safe + (is_gradual && is_strictly_monotonic) as u16
        },
    );

    println!("{}", num_safe);
}
