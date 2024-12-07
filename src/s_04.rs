use std::fmt::Display;

use ndarray::Array2;

use crate::common::*;

pub struct S;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Character {
    Wildcard = 0,
    X = 1,
    M = 2,
    A = 3,
    S = 4,
}

impl Character {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Wildcard,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wildcard => write!(f, "."),
            Self::X => write!(f, "X"),
            Self::M => write!(f, "M"),
            Self::A => write!(f, "A"),
            Self::S => write!(f, "S"),
        }
    }
}

pub struct Grid2d<T> {
    width: usize,
    height: usize,
    array: Array2<T>,
}

impl<T> Grid2d<T>
where
    T: Clone,
{
    fn window2d(&self, shape: (usize, usize)) -> impl Iterator<Item = Grid2d<T>> + use<'_, T> {
        self.array.windows(shape).into_iter().map(move |w| Grid2d {
            width: shape.0,
            height: shape.1,
            array: w.to_owned(),
        })
    }
}

impl Grid2d<Character> {
    fn new(string: &str) -> Self {
        let lines: Vec<String> = string.lines().map(|s| s.to_string()).collect();
        let width = lines[0].len();
        let height = lines.len();
        let elements = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Character::from_char)
            .collect::<Vec<_>>();

        let shape = (height, width);

        let array = Array2::from_shape_vec(shape, elements).expect("Shape mismatch");
        Self {
            width,
            height,
            array,
        }
    }

    fn matches(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        self.array
            .iter()
            .zip(other.array.iter())
            .all(|(a, b)| b == &Character::Wildcard || a == b)
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = Grid2d::new(&input.input);

        let horizontal = Grid2d::new("XMAS");
        let horizontal_r = Grid2d::new("SAMX");
        let vertical = Grid2d::new("X\nM\nA\nS");
        let vertical_r = Grid2d::new("S\nA\nM\nX");
        let diagonal_1 = Grid2d::new("X...\n.M..\n..A.\n...S");
        let diagonal_2 = Grid2d::new("S...\n.A..\n..M.\n...X");
        let diagonal_3 = Grid2d::new("...X\n..M.\n.A..\nS...");
        let diagonal_4 = Grid2d::new("...S\n..A.\n.M..\nX...");

        let mut result = 0;

        for w in grid.window2d((4, 1)) {
            if w.matches(&horizontal) || w.matches(&horizontal_r) {
                result += 1;
            }
        }

        for w in grid.window2d((1, 4)) {
            if w.matches(&vertical) || w.matches(&vertical_r) {
                result += 1;
            }
        }

        for w in grid.window2d((4, 4)) {
            if w.matches(&diagonal_1)
                || w.matches(&diagonal_2)
                || w.matches(&diagonal_3)
                || w.matches(&diagonal_4)
            {
                result += 1;
            }
        }

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
    }

    fn expected_output_one(&self) -> &str {
        "18"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = Grid2d::new(&input.input);

        let mas_1 = Grid2d::new("M.S\n.A.\nM.S");
        let mas_2 = Grid2d::new("S.M\n.A.\nS.M");
        let mas_3 = Grid2d::new("S.S\n.A.\nM.M");
        let mas_4 = Grid2d::new("M.M\n.A.\nS.S");

        let mut result = 0;

        for w in grid.window2d((3, 3)) {
            if w.matches(&mas_1) || w.matches(&mas_2) || w.matches(&mas_3) || w.matches(&mas_4) {
                result += 1;
            }
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
    }

    fn expected_output_two(&self) -> &str {
        "9"
    }
}
