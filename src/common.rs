#![allow(dead_code)]
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{BufRead, BufReader, Lines, Result};
use std::ops::{Add, Div, Mul, Sub};
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
        Grid2d::from_lines(&self.lines, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2d<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>,
}

impl<T> Grid2d<T> {
    pub fn new(width: usize, height: usize, vec: Vec<T>) -> Self {
        Self { width, height, vec }
    }

    pub fn from_lines(lines: &[String], f: fn(char) -> T) -> Grid2d<T> {
        let width = lines[0].len();
        let height = lines.len();
        let vec = lines
            .iter()
            .flat_map(|line| line.chars().map(f).collect::<Vec<T>>())
            .collect();
        Grid2d::new(width, height, vec)
    }

    pub fn to_pos(&self, index: i32) -> Option<VecI2> {
        if index < 0 || index >= self.width as i32 * self.height as i32 {
            return None;
        }
        let x = index % self.width as i32;
        let y = index / self.width as i32;
        Some(VecI2(x, y))
    }

    pub fn in_bounds(&self, pos: &VecI2) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width as i32 && pos.1 < self.height as i32
    }

    pub fn to_index(&self, pos: &VecI2) -> Option<usize> {
        if !self.in_bounds(pos) {
            return None;
        }
        Some(pos.0 as usize + pos.1 as usize * self.width)
    }

    pub fn get(&self, pos: &VecI2) -> Option<&T> {
        let index = self.to_index(pos)?;
        self.vec.get(index)
    }

    pub fn set(&mut self, pos: &VecI2, value: T) -> Option<()> {
        let index = self.to_index(pos)?;
        if let Some(a) = self.vec.get_mut(index) {
            *a = value;
            Some(())
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (VecI2, &T)> {
        self.vec
            .iter()
            .enumerate()
            .map(|(i, t)| (VecI2((i % self.width) as i32, (i / self.width) as i32), t))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VecI2, &mut T)> {
        self.vec
            .iter_mut()
            .enumerate()
            .map(|(i, t)| (VecI2((i % self.width) as i32, (i / self.width) as i32), t))
    }

    pub fn find_first(&self, f: impl Fn(&T) -> bool) -> Option<(VecI2, &T)> {
        self.iter().find(|(_, t)| f(t))
    }
}

impl<T: Display> Display for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec.iter().enumerate().for_each(|(i, t)| {
            write!(f, "{t}").unwrap();
            if i % self.width == self.width - 1 {
                writeln!(f).unwrap();
            }
        });
        Ok(())
    }
}

pub struct DisplayVec<T: Display>(pub Vec<T>);

impl<T: Display> Display for DisplayVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VecI2(pub i32, pub i32);

impl VecI2 {
    pub fn up(&self) -> Self {
        Self(self.0, self.1 - 1)
    }
    pub fn right(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
    pub fn down(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
    pub fn left(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    pub fn min(&self, other: &VecI2) -> VecI2 {
        VecI2(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn max(&self, other: &VecI2) -> VecI2 {
        VecI2(self.0.max(other.0), self.1.max(other.1))
    }
}

impl Add for VecI2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for VecI2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for VecI2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<i32> for VecI2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
