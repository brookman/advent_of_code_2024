use crate::common::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

pub struct S {}

#[derive(Clone, Debug)]
enum OperationType {
    Mul(i32, i32),
    Do,
    Dont,
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
        let operations = parse_operations(&input.input).unwrap().1;

        let result: i32 = operations
            .iter()
            .map(|op| match op {
                OperationType::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum();

        Some(format!("{}", result))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let operations = parse_operations(&input.input).unwrap().1;

        let mut result = 0;
        let mut enabled = true;

        for op in operations {
            match op {
                OperationType::Mul(a, b) => {
                    if enabled {
                        result += a * b
                    }
                }
                OperationType::Do => enabled = true,
                OperationType::Dont => enabled = false,
            }
        }

        Some(format!("{}", result))
    }
}

fn parse_operations(input: &str) -> IResult<&str, Vec<OperationType>> {
    map(
        many0(alt((
            map(alt((parse_mul, parse_do, parse_dont)), Some),
            map(consume_invalid, |_| None),
        ))),
        |opt_vec| opt_vec.into_iter().flatten().collect(),
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, OperationType> {
    let operands = delimited(
        tag("mul("),
        separated_pair(parse_number, tag(","), parse_number),
        tag(")"),
    );
    map(operands, |(a, b)| OperationType::Mul(a as i32, b as i32))(input)
}

fn parse_number(input: &str) -> IResult<&str, u16> {
    let digits = take_while_m_n(1, 3, |c: char| c.is_ascii_digit());
    map_res(digits, |s: &str| s.parse::<u16>())(input)
}

fn parse_do(input: &str) -> IResult<&str, OperationType> {
    map(tag("do()"), |_| OperationType::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, OperationType> {
    map(tag("don't()"), |_| OperationType::Dont)(input)
}

fn consume_invalid(input: &str) -> IResult<&str, ()> {
    map(anychar, |_| ())(input)
}
