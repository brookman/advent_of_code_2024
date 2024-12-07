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

    pub fn grid2d<T>(&self, f: fn(char) -> T) -> Grid2d<T> {
        let width = self.lines[0].len();
        let height = self.lines.len();
        let vec = self
            .lines
            .iter()
            .flat_map(|line| line.chars().map(f).collect::<Vec<T>>())
            .collect();
        Grid2d::new(width, height, vec)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2d<T> {
    width: usize,
    height: usize,
    vec: Vec<T>,
}

impl<T> Grid2d<T> {
    fn new(width: usize, height: usize, vec: Vec<T>) -> Self {
        Self { width, height, vec }
    }

    fn to_pos(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.width as i32 * self.height as i32 {
            return None;
        }
        let x = index % self.width as i32;
        let y = index / self.width as i32;
        Some((x, y))
    }

    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width as i32 && pos.1 < self.height as i32
    }

    fn to_index(&self, pos: (i32, i32)) -> Option<usize> {
        if !self.in_bounds(pos) {
            return None;
        }
        Some(pos.0 as usize + pos.1 as usize * self.width)
    }

    fn get(&self, pos: (i32, i32)) -> Option<&T> {
        let index = self.to_index(pos)?;
        self.vec.get(index)
    }

    fn set(&mut self, pos: (i32, i32), value: T) -> Option<()> {
        let index = self.to_index(pos)?;
        if let Some(a) = self.vec.get_mut(index) {
            *a = value;
            Some(())
        } else {
            None
        }
    }

    fn iter(&self) -> impl Iterator<Item = (i32, i32, &T)> {
        self.vec
            .iter()
            .enumerate()
            .map(|(i, t)| ((i % self.width) as i32, (i / self.width) as i32, t))
    }

    fn find_first(&self, f: impl Fn(&T) -> bool) -> Option<(i32, i32, &T)> {
        self.iter().find(|(_, _, t)| f(t))
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
