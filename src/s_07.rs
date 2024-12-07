use std::iter::repeat;

use itertools::Itertools;
use rayon::prelude::*;

use crate::common::*;

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        const OPERATORS: [char; 2] = ['+', '*'];
        solve(input, &OPERATORS).to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
    }

    fn expected_output_one(&self) -> &str {
        "3749"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        const OPERATORS: [char; 3] = ['+', '*', '|'];
        solve(input, &OPERATORS).to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
    }

    fn expected_output_two(&self) -> &str {
        "11387"
    }
}

fn solve(input: &PuzzleInput, operators: &[char]) -> u64 {
    input
        .lines
        .par_iter()
        .map(|line| {
            let mut parts = line.split(':');
            let test_value = parts.next().unwrap().parse::<u64>().unwrap();
            let rem = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect::<Vec<_>>();
            test_value_or_zero(operators, &rem, test_value)
        })
        .sum()
}

fn test_value_or_zero(operators: &[char], rem: &[u16], test_value: u64) -> u64 {
    for op_combinations in repeat(operators.iter())
        .take(rem.len() - 1)
        .multi_cartesian_product()
    {
        let mut res = rem[0] as u64;
        for (i, op) in op_combinations.iter().enumerate() {
            let a = rem[i + 1] as u64;
            match op {
                '+' => res += a,
                '*' => res *= a,
                '|' => res = res * 10u64.pow(a.ilog10() + 1) + a,
                _ => panic!("Unknown operator"),
            }
            if res >= test_value {
                break;
            }
        }

        if res == test_value {
            return test_value;
        }
    }
    0
}
