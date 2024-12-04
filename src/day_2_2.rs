use std::fs;

/// https://adventofcode.com/2024/day/2#part2
pub fn day_2_2() {
    let data = fs::read_to_string("data/day_2.txt").expect("missing file");

    let num_safe = data.split('\n').filter(|line| !line.is_empty()).fold(
        0,
        |num_safe, report| {
            let levels: Vec<_> = report
                .split(" ")
                .filter_map(|level| level.parse::<i8>().ok())
                .collect();

            let is_safe = (0..=levels.len()).any(|skip_level| {
                let levels_sublist = [
                    &levels[..skip_level.saturating_sub(1)],
                    &levels[skip_level..],
                ]
                .concat();

                let (is_gradual, directions_sum) = levels_sublist
                    .windows(2)
                    .flat_map(<&[i8; 2]>::try_from)
                    .fold(
                        (true, 0),
                        |(is_gradual, directions_sum), &[level_a, level_b]| {
                            (
                                is_gradual
                                    && (1..=3)
                                        .contains(&level_b.abs_diff(level_a)),
                                directions_sum + (level_b - level_a).signum(),
                            )
                        },
                    );

                let abs_directions_sum: usize =
                    directions_sum.abs_diff(0).into();
                let is_strictly_monotonic = abs_directions_sum
                    == levels_sublist.len().saturating_sub(1);

                is_gradual && is_strictly_monotonic
            });

            num_safe + is_safe as u16
        },
    );

    println!("{}", num_safe);
}
