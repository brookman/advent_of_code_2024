use std::{collections::HashMap, fmt::Display};

use crate::common::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use regex::Regex;

pub struct S;

const register_a: usize = 0;
const register_b: usize = 1;
const register_c: usize = 2;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Register {
    A,
    B,
    C,
}

impl Register {
    fn from_i32(value: i32) -> Self {
        match value {
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Unknown value: {}", value),
        }
    }

    fn from_char(value: char) -> Self {
        match value {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Operand {
    Literal(u64),
    Combo(Register),
    Reserved,
}

impl Operand {
    fn combo(value: u64) -> Self {
        match value {
            0 => Self::Literal(value),
            1 => Self::Literal(value),
            2 => Self::Literal(value),
            3 => Self::Literal(value),
            4 => Self::Combo(Register::A),
            5 => Self::Combo(Register::B),
            6 => Self::Combo(Register::C),
            7 => Self::Reserved,
            _ => panic!("Unknown value: {}", value),
        }
    }

    fn literal(value: u64) -> Self {
        match value {
            0 => Self::Literal(value),
            1 => Self::Literal(value),
            2 => Self::Literal(value),
            3 => Self::Literal(value),
            4 => Self::Literal(value),
            5 => Self::Literal(value),
            6 => Self::Literal(value),
            7 => Self::Reserved,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

lazy_static! {
    static ref REGISTER: Regex = Regex::new(r"^Register (.+?): (\d+)$").unwrap();
    static ref PROGRAM: Regex = Regex::new(r"^Program: (.+)$").unwrap();
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let mut registers = HashMap::new();
        let mut program = vec![];

        for line in input.lines.iter() {
            for cap in REGISTER.captures_iter(line) {
                let register = cap.get(1).unwrap().as_str();
                let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                registers.insert(Register::from_char(register.chars().next().unwrap()), value);
            }
            for cap in PROGRAM.captures_iter(line) {
                let program_str = cap.get(1).unwrap().as_str();
                program = program_str
                    .split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            }
        }

        println!("{:?}", registers);
        println!("{:?}", program);

        let mut output = vec![];

        let mut ip = 0;

        loop {
            if ip >= program.len() {
                break;
            }
            let instruction = program[ip];
            match instruction {
                0 => {
                    // division
                    let operand = Operand::combo(program[ip + 1]);
                    let a = registers[&Register::A];
                    let o = match operand {
                        Operand::Literal(value) => value,
                        Operand::Combo(register) => registers[&register],
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    registers.insert(Register::A, a / 2u64.pow(o as u32));
                    ip += 2;
                }
                1 => {
                    // bitwise xor
                    let operand = Operand::literal(program[ip + 1]);
                    let b = registers[&Register::B];
                    let o = match operand {
                        Operand::Literal(value) => value,
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    registers.insert(Register::B, b ^ o);
                    ip += 2;
                }
                2 => {
                    // modulo 8
                    let operand = Operand::combo(program[ip + 1]);
                    let o = match operand {
                        Operand::Literal(value) => value,
                        Operand::Combo(register) => registers[&register],
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    registers.insert(Register::B, o % 8);
                    ip += 2;
                }
                3 => {
                    // jump not zero
                    let a = registers[&Register::A];
                    if a != 0 {
                        let operand = Operand::literal(program[ip + 1]);
                        let o = match operand {
                            Operand::Literal(value) => value,
                            _ => panic!("Unexpected operand: {:?}", operand),
                        };
                        ip = o as usize;
                    } else {
                        ip += 2;
                    }
                }
                4 => {
                    // xor
                    let b = registers[&Register::B];
                    let c = registers[&Register::C];
                    registers.insert(Register::B, b ^ c);
                    ip += 2;
                }
                5 => {
                    let operand = Operand::combo(program[ip + 1]);
                    let o = match operand {
                        Operand::Literal(value) => value,
                        Operand::Combo(register) => registers[&register],
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    output.push(o % 8);
                    ip += 2;
                }
                6 => {
                    // division
                    let operand = Operand::combo(program[ip + 1]);
                    let a = registers[&Register::A];
                    let o = match operand {
                        Operand::Literal(value) => value,
                        Operand::Combo(register) => registers[&register],
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    registers.insert(Register::B, a / 2u64.pow(o as u32));
                    ip += 2;
                }
                7 => {
                    // division
                    let operand = Operand::combo(program[ip + 1]);
                    let a = registers[&Register::A];
                    let o = match operand {
                        Operand::Literal(value) => value,
                        Operand::Combo(register) => registers[&register],
                        _ => panic!("Unexpected operand: {:?}", operand),
                    };
                    registers.insert(Register::C, a / 2u64.pow(o as u32));
                    ip += 2;
                }
                _ => panic!("Unknown instruction: {}", instruction),
            }
        }

        let result = output.iter().join(",");

        result
    }

    fn test_input_one(&self) -> &str {
        r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#
    }

    fn expected_output_one(&self) -> &str {
        "4,6,3,5,6,3,5,2,1,0"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let mut registers = [0u64; 3];
        let mut program = vec![];

        let mut register_index = 0;
        for line in input.lines.iter() {
            for cap in REGISTER.captures_iter(line) {
                let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                registers[register_index] = value;
                register_index += 1;
            }
            for cap in PROGRAM.captures_iter(line) {
                let program_str = cap.get(1).unwrap().as_str();
                program = program_str
                    .split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            }
        }

        println!("{:?}", registers);
        println!("{:?}", program);

        //let start = 0u64;
        let start = 2_077_600_000_000u64;

        let result = (start..259_700_000_000u64 * 8)
            .into_par_iter()
            .find_first(|initial_a| {
                let mut registers = registers;
                registers[register_a] = *initial_a;

                if initial_a % 10000000 == 0 {
                    println!("Testing {}, registers: {:?}", initial_a, registers);
                }

                let mut output = vec![];
                let mut ip = 0;

                loop {
                    let literal = || program[ip + 1];
                    let combo = || {
                        let value = program[ip + 1];
                        match value {
                            0..=3 => value,
                            4..=6 => registers[(value - 4) as usize],
                            _ => panic!("Unknown value: {}", value),
                        }
                    };

                    if ip >= program.len() {
                        break;
                    }
                    let instruction = program[ip];
                    match instruction {
                        0 => {
                            // division
                            let a = registers[register_a];
                            let o = combo();
                            registers[register_a] = a / 2u64.pow(o as u32);
                            ip += 2;
                        }
                        1 => {
                            // bitwise xor
                            let b = registers[register_b];
                            let o = literal();
                            registers[register_b] = b ^ o;
                            ip += 2;
                        }
                        2 => {
                            // modulo 8
                            let o = combo();
                            registers[register_b] = o % 8;
                            ip += 2;
                        }
                        3 => {
                            // jump not zero
                            let a = registers[register_a];
                            if a != 0 {
                                let o = literal();
                                ip = o as usize;
                            } else {
                                ip += 2;
                            }
                        }
                        4 => {
                            // xor
                            let b = registers[register_b];
                            let c = registers[register_c];
                            registers[register_b] = b ^ c;
                            ip += 2;
                        }
                        5 => {
                            let o = combo();
                            output.push(o % 8);
                            if !program.starts_with(&output) {
                                break;
                            }
                            ip += 2;
                        }
                        6 => {
                            // division
                            let a = registers[register_a];
                            let o = combo();
                            registers[register_b] = a / 2u64.pow(o as u32);
                            ip += 2;
                        }
                        7 => {
                            // division
                            let a = registers[register_a];
                            let o = combo();
                            registers[register_c] = a / 2u64.pow(o as u32);
                            ip += 2;
                        }
                        _ => panic!("Unknown instruction: {}", instruction),
                    }
                }

                program == output
            })
            .unwrap();

        println!("{}", result);

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#
    }

    fn expected_output_two(&self) -> &str {
        // "117440"
        ""
    }
}
