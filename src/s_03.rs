use std::vec;

use crate::common::*;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref DO: Regex = Regex::new(r"do\(\)").unwrap();
    static ref DONT: Regex = Regex::new(r"don't\(\)").unwrap();
}

#[derive(Clone)]
enum OperationType {
    Mul(i32, i32),
    Do,
    Dont,
}

#[derive(Clone)]
struct Operation {
    position: usize,
    operation: OperationType,
}

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let operations = parse_operations(input);

        let result: i32 = operations
            .iter()
            .map(|op| match op.operation {
                OperationType::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum();

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn expected_output_one(&self) -> &str {
        "161"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let operations = parse_operations(input);

        let mut result = 0;
        let mut enabled = true;

        for op in operations {
            match op.operation {
                OperationType::Mul(a, b) => {
                    if enabled {
                        result += a * b
                    }
                }
                OperationType::Do => enabled = true,
                OperationType::Dont => enabled = false,
            }
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    fn expected_output_two(&self) -> &str {
        "48"
    }
}

fn parse_operations(input: &PuzzleInput) -> Vec<Operation> {
    let mut operations = vec![];
    for line in input.lines.iter() {
        let mut line_operations = vec![];
        for mul in MUL.captures_iter(line) {
            let num = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            line_operations.push(Operation {
                position: mul.get(0).unwrap().start(),
                operation: OperationType::Mul(num, num2),
            });
        }
        for d in DO.captures_iter(line) {
            line_operations.push(Operation {
                position: d.get(0).unwrap().start(),
                operation: OperationType::Do,
            });
        }
        for dont in DONT.captures_iter(line) {
            line_operations.push(Operation {
                position: dont.get(0).unwrap().start(),
                operation: OperationType::Dont,
            });
        }
        line_operations.sort_by(|a, b| a.position.cmp(&b.position));
        operations.extend_from_slice(&line_operations);
    }

    operations
}
