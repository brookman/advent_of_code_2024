#![allow(dead_code)]
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Lines, Result};
use std::str::FromStr;
use std::{fs::File, path::Path};

pub fn read_parsed<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_lines(filename)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

pub fn parsed<T>(lines: &Vec<String>) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines.iter().map(|s| s.parse::<T>().unwrap()).collect()
}

pub fn read_strings(filename: &str) -> Vec<String> {
    read_lines(filename)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
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

pub trait Solution {
    fn solve_one(&self, input: &str, lines: &[&str]) -> Option<String>;
    fn solve_two(&self, input: &str, lines: &[&str]) -> Option<String>;
}
