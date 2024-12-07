use itertools::Itertools;

use crate::common::*;

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let operators = ['+', '*'];
        let mut result = 0;
        for line in &input.lines {
            let mut parts = line.split(':');
            let test_value = parts.next().unwrap().parse::<u64>().unwrap();
            let rem = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            for op_combinations in std::iter::repeat(operators.iter())
            .take(rem.len() - 1)
            .multi_cartesian_product()
            {
                let mut res = rem[0];
                for (i, op) in op_combinations.iter().enumerate() {
                    let a = rem[i + 1];
                    match op {
                        '+' => res += a,
                        '*' => res *= a,
                        _ => panic!("Unknown operator"),
                    }
                }
                if res == test_value {
                    result += test_value;
                    break;
                }
            }
        }

        result.to_string()
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
        let operators = ['+', '*', '|'];
        let mut result = 0;
        for line in &input.lines {
            let mut parts = line.split(':');
            let test_value = parts.next().unwrap().parse::<u64>().unwrap();
            let rem = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            for op_combinations in std::iter::repeat(operators.iter())
            .take(rem.len() - 1)
            .multi_cartesian_product()
            {
                let mut res = rem[0];
                for (i, op) in op_combinations.iter().enumerate() {
                    let a = rem[i + 1];
                    match op {
                        '+' => res += a,
                        '*' => res *= a,
                        '|' => res = format!("{res}{a}").parse::<u64>().unwrap(),
                        _ => panic!("Unknown operator"),
                    }
                }
                if res == test_value {
                    result += test_value;
                    break;
                }
            }
        }

        result.to_string()
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
