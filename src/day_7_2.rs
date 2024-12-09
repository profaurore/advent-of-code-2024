use std::fs;

fn num_concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse::<u64>().unwrap()
}

const OPS: [fn(u64, u64) -> u64; 3] =
    [num_concat, u64::saturating_mul, u64::saturating_add];

/// https://adventofcode.com/2024/day/7#part2
pub fn day_7_2() {
    let data = fs::read_to_string("data/day_7.txt").expect("missing file");
    let lines = data.split('\n');

    let valid_test_values = lines
        .filter_map(|line| {
            line.split_once(": ")
                .and_then(|(test_value_str, operands_str)| {
                    test_value_str.parse::<u64>().ok().map(|test_value| {
                        (
                            test_value,
                            operands_str
                                .split(' ')
                                .filter_map(|operand| {
                                    operand.parse::<u64>().ok()
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                })
        })
        .fold(0, |valid, (test_value, operands)| {
            if operands.is_empty() {
                return valid;
            }

            let num_ops = OPS.len();
            let num_operands = operands.len();
            let mut operators: Vec<usize> = vec![num_ops - 1; num_operands - 1];

            loop {
                let total = operators
                    .iter()
                    .zip(operands.iter().skip(1))
                    .fold(operands[0], |total, (&op_index, &value)| {
                        OPS[op_index](total, value)
                    });

                if total == test_value {
                    return valid + test_value;
                }

                if operators.iter().sum::<usize>() == 0 {
                    break;
                }

                for operator in operators.iter_mut().rev() {
                    if *operator == 0 {
                        *operator = num_ops - 1;
                    } else {
                        *operator -= 1;
                        break;
                    }
                }
            }

            valid
        });

    println!("{}", valid_test_values);
}
