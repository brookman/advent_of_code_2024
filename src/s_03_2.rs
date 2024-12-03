use crate::common::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_res},
    error::ParseError,
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult, InputLength, Parser,
};

type R<'a, T> = IResult<&'a str, T>;
type Context<'a> = (&'a str, (bool, u32));

pub struct S {}

#[derive(Clone, Debug)]
enum OperationType {
    Mul(u32, u32),
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
        let ops = alt((map(parse_mul, Some), consume_invalid));
        let result = parse_and_sum(&input.input, ops);
        Some(format!("{}", result))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let ops = alt((
            map(alt((parse_mul, parse_do, parse_dont)), Some),
            consume_invalid,
        ));
        let result = parse_and_sum(&input.input, ops);
        Some(format!("{}", result))
    }
}

fn parse_and_sum<'a>(input: &'a str, f: impl FnMut(&'a str) -> R<Option<OperationType>>) -> u32 {
    fold_many0(
        f,
        || (1, 0),
        |(enabled, result), op| match op {
            Some(OperationType::Mul(a, b)) => (enabled, result + enabled * a * b),
            Some(OperationType::Do) => (1, result),
            Some(OperationType::Dont) => (0, result),
            _ => (enabled, result),
        },
    )(input)
    .unwrap()
    .1
     .1
}

fn parse_mul(input: &str) -> R<OperationType> {
    let operands = delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    );
    map(operands, |(a, b)| OperationType::Mul(a as u32, b as u32))(input)
}

fn parse_number(input: &str) -> R<u16> {
    let digits = take_while_m_n(1, 3, |c: char| c.is_ascii_digit());
    map_res(digits, |s: &str| s.parse::<u16>())(input)
}

fn parse_do(input: &str) -> R<OperationType> {
    map(tag("do()"), |_| OperationType::Do)(input)
}

fn parse_dont(input: &str) -> R<OperationType> {
    map(tag("don't()"), |_| OperationType::Dont)(input)
}

fn consume_invalid(input: &str) -> R<Option<OperationType>> {
    map(anychar, |_| None)(input)
}
