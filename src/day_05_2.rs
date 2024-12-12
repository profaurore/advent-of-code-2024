use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn is_update_correct(
    update: &str,
    page_ordering_rules: &HashMap<u32, HashSet<u32>>,
) -> bool {
    let mut added_set: HashSet<u32> = HashSet::new();
    let mut added_list: Vec<u32> = Vec::new();

    update
        .split(',')
        .filter_map(|page| page.parse::<u32>().ok())
        .rev()
        .all(|page| {
            added_set.insert(page);
            added_list.push(page);

            page_ordering_rules
                .get(&page)
                .is_none_or(|preceeders| preceeders.is_disjoint(&added_set))
        })
}

/// https://adventofcode.com/2024/day/5#part2
pub fn day_05_2() {
    let data = fs::read_to_string("data/day_05.txt").expect("missing file");
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
                let is_correct =
                    is_update_correct(update, &page_ordering_rules);

                sum + if !is_correct {
                    let corrected_update = update
                        .split(',')
                        .filter_map(|page| page.parse::<u32>().ok())
                        .fold(Vec::new(), |mut update, page| {
                            let update_len = update.len();

                            let index = page_ordering_rules
                                .get(&page)
                                .and_then(|preceeders| {
                                    update
                                        .iter()
                                        .rev()
                                        .position(|update_page| {
                                            preceeders.contains(update_page)
                                        })
                                        .map(|rev_index| update_len - rev_index)
                                })
                                .unwrap_or(0);

                            update.insert(index, page);

                            update
                        });

                    corrected_update[corrected_update.len() / 2]
                } else {
                    0
                }
            });

    println!("{}", middles_sum);
}
