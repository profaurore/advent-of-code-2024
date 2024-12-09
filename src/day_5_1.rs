use std::{
    collections::{HashMap, HashSet},
    fs,
};

/// https://adventofcode.com/2024/day/5#part1
pub fn day_5_1() {
    let data = fs::read_to_string("data/day_5.txt").expect("missing file");
    let mut lines = data.split('\n');

    // Page ordering rules
    let page_ordering_rules: HashMap<u32, HashSet<u32>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .fold(HashMap::new(), |mut rules, rule| {
            if let Some((page_a, page_b)) = rule.split_once('|') {
                if let (Ok(page_a), Ok(page_b)) =
                    (page_a.parse::<u32>(), page_b.parse::<u32>())
                {
                    rules.entry(page_b).or_default().insert(page_a);
                }
            }

            rules
        });

    // Pages to produce
    let middles_sum =
        lines
            .filter(|line| !line.is_empty())
            .fold(0, |sum, update| {
                let mut added_set: HashSet<u32> = HashSet::new();
                let mut added_list: Vec<u32> = Vec::new();

                let is_correct = update
                    .split(',')
                    .filter_map(|page| page.parse::<u32>().ok())
                    .rev()
                    .all(|page| {
                        added_set.insert(page);
                        added_list.push(page);

                        page_ordering_rules.get(&page).is_none_or(
                            |preceeders| preceeders.is_disjoint(&added_set),
                        )
                    });

                sum + if is_correct {
                    added_list[added_list.len() / 2]
                } else {
                    0
                }
            });

    println!("{}", middles_sum);
}
