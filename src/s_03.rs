use std::vec;

use crate::common::*;

use lazy_static::lazy_static;
use regex::Regex;

pub struct S {}

lazy_static! {
    static ref MUL: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref DO: Regex = Regex::new(r"do\(\)").unwrap();
    static ref DONT: Regex = Regex::new(r"don't\(\)").unwrap();
}

enum OperationType {
    Mul(i32, i32),
    Do,
    Dont,
}

struct Operation {
    position: usize,
    operation: OperationType,
}

impl Solution for S {
    fn test_one(&self) -> (&str, &str) {
        (
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#,
            "161",
        )
    }

    fn test_two(&self) -> (&str, &str) {
        (
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#,
            "48",
        )
    }

    fn solve_one(&self, input: &PuzzleInput) -> Option<String> {
        let mut result = 0;

        for line in input.lines.iter() {
            if MUL.is_match(line) {
                let caps = MUL.captures_iter(line);
                for cap in caps {
                    //println!("{:?}", cap);
                    let num = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let num2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    result += num * num2;
                }
            }
        }

        Some(format!("{}", result))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let mut result = 0;
        let mut enabled = true;

        for line in input.lines.iter() {
            let mut operations = vec![];

            let muls = MUL.captures_iter(line);
            for mul in muls {
                let num = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let num2 = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
                operations.push(Operation {
                    position: mul.get(0).unwrap().start(),
                    operation: OperationType::Mul(num, num2),
                });
            }
            let dos = DO.captures_iter(line);
            for d in dos {
                operations.push(Operation {
                    position: d.get(0).unwrap().start(),
                    operation: OperationType::Do,
                });
            }
            let donts = DONT.captures_iter(line);
            for dont in donts {
                operations.push(Operation {
                    position: dont.get(0).unwrap().start(),
                    operation: OperationType::Dont,
                });
            }

            operations.sort_by(|a, b| a.position.cmp(&b.position));
           
            for op in operations {
                match op.operation {
                    OperationType::Mul(num, num2) => {
                        if enabled {
                            result += num * num2;
                        }
                    }
                    OperationType::Do => {
                        enabled = true;
                    }
                    OperationType::Dont => {
                        enabled = false;
                    }
                }
            }
        }

        Some(format!("{}", result))
    }
}
