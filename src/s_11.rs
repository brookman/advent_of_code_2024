use std::collections::HashMap;

use crate::common::*;

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        solve(input, 25).to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"125 17
"#
    }

    fn expected_output_one(&self) -> &str {
        "55312"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        solve(input, 75).to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"125 17
"#
    }

    fn expected_output_two(&self) -> &str {
        "65601038650482"
    }
}

fn solve(input: &PuzzleInput, blinks: usize) -> usize {
    let mut stones = input.lines[0]
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0usize) += 1;
            map
        });

    for _ in 0..blinks {
        let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len() * 2);

        for (key, count) in stones {
            let mut update = |n| {
                *new_stones.entry(n).or_insert(0usize) += count;
            };
            if key == 0 {
                update(1);
            } else {
                let number_of_digits = if key == 0 { 1 } else { key.ilog10() + 1 };
                if number_of_digits % 2 == 0 {
                    let half = (number_of_digits / 2) as usize;
                    let mask = 10usize.pow(half as u32);
                    update(key / mask);
                    update(key % mask);
                } else {
                    update(key * 2024);
                }
            }
        }
        stones = new_stones;
    }

    stones.values().sum::<usize>()
}
