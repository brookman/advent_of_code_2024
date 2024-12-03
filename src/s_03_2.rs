use crate::common::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_res},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

type R<'a, T> = IResult<&'a str, T>;

#[derive(Clone, Debug)]
enum Operation {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Clone, Debug)]
struct Context {
    enabled: bool,
    sum: u32,
}

impl Context {
    fn new() -> Self {
        Self {
            enabled: true,
            sum: 0,
        }
    }

    pub fn add(mut self, op: Option<Operation>) -> Self {
        if let Some(op) = op {
            match op {
                Operation::Mul(a, b) => {
                    if self.enabled {
                        self.sum += a * b
                    }
                }
                Operation::Do => self.enabled = true,
                Operation::Dont => self.enabled = false,
            }
        }
        self
    }
}

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let operations = parse_mul;
        parse_operations_and_sum(&input.input, operations).to_string()
    }

    fn test_input_one(&self) -> &str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn expected_output_one(&self) -> &str {
        "161"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let operations = alt((parse_mul, parse_do, parse_dont));
        parse_operations_and_sum(&input.input, operations).to_string()
    }

    fn test_input_two(&self) -> &str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    fn expected_output_two(&self) -> &str {
        "48"
    }
}

fn parse_operations_and_sum<'a>(
    input: &'a str,
    operation_parser: impl FnMut(&'a str) -> IResult<&'a str, Operation>,
) -> u32 {
    let context = fold_many0(
        // either an operation (-> Some) or anychar else (-> None)
        alt((map(operation_parser, Some), map(anychar, |_| None))),
        Context::new,
        Context::add,
    )(input)
    .unwrap()
    .1;
    context.sum
}

fn parse_mul(input: &str) -> R<Operation> {
    let operands = delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    );
    map(operands, |(a, b)| Operation::Mul(a as u32, b as u32))(input)
}

fn parse_number(input: &str) -> R<u16> {
    let digits = take_while_m_n(1, 3, |c: char| c.is_ascii_digit());
    map_res(digits, |s: &str| s.parse::<u16>())(input)
}

fn parse_do(input: &str) -> R<Operation> {
    map(tag("do()"), |_| Operation::Do)(input)
}

fn parse_dont(input: &str) -> R<Operation> {
    map(tag("don't()"), |_| Operation::Dont)(input)
}
