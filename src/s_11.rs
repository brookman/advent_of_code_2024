use std::mem;

use rustc_hash::FxHashMap;

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
        .fold(FxHashMap::default(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });

    let mut new_stones = FxHashMap::default();
    new_stones.reserve(3771);

    for _ in 0..blinks {
        new_stones.clear();

        for (num, count) in stones.iter() {
            if *num == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else {
                let number_of_digits = (*num as f32).log10().trunc() as u32 + 1;
                if number_of_digits % 2 == 0 {
                    let mask = 10usize.pow(number_of_digits / 2);
                    *new_stones.entry(num / mask).or_insert(0) += count;
                    *new_stones.entry(num % mask).or_insert(0) += count;
                } else {
                    *new_stones.entry(num * 2024).or_insert(0) += count;
                }
            }
        }
        mem::swap(&mut stones, &mut new_stones);
    }

    stones.values().sum::<usize>()
}
