#![allow(dead_code)]
use std::fmt::Debug;
use std::fs;
use std::io::{BufRead, BufReader, Lines, Result};
use std::str::FromStr;
use std::{fs::File, path::Path};

pub fn read_strings(filename: &str) -> Vec<String> {
    read_lines(filename)
        .unwrap()
        .map_while(Result::ok)
        .collect()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub struct PuzzleInput {
    pub input: String,
    pub lines: Vec<String>,
}

impl PuzzleInput {
    pub fn new(file_path: &str) -> Option<Self> {
        let input = fs::read_to_string(file_path).unwrap_or_default();
        if input.is_empty() {
            return None;
        }
        let lines = input.lines().map(|s| s.to_string()).collect();
        Some(Self {
            input: input.to_string(),
            lines,
        })
    }

    pub fn from_str(input: &str) -> Option<Self> {
        let lines = input.lines().map(|s| s.to_string()).collect();
        Some(Self {
            input: input.to_string(),
            lines,
        })
    }

    pub fn parsed<T>(&self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.lines.iter().map(|s| s.parse::<T>().unwrap()).collect()
    }

    pub fn parsed2d<T>(&self) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<T>().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

pub trait Solution {
    fn solve_one(&self, input: &PuzzleInput) -> String;
    fn test_input_one(&self) -> &str {
        ""
    }
    fn expected_output_one(&self) -> &str {
        ""
    }

    fn solve_two(&self, input: &PuzzleInput) -> String;
    fn test_input_two(&self) -> &str {
        ""
    }
    fn expected_output_two(&self) -> &str {
        ""
    }
}
