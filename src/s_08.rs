use std::fmt::Display;

use itertools::Itertools;

use crate::common::*;

pub struct S;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    EmptyWithAntinode,
    Antenna(char),
    AntennaWithAntinode(char),
}

impl Location {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            _ => Self::Antenna(c),
        }
    }

    fn is_antenna(&self) -> bool {
        matches!(self, Self::Antenna(_) | Self::AntennaWithAntinode(_))
    }

    fn is_same_antenna_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Antenna(c1), Self::Antenna(c2)) => c1 == c2,
            (Self::AntennaWithAntinode(c1), Self::AntennaWithAntinode(c2)) => c1 == c2,
            _ => false,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Antenna(c) => write!(f, "{}", c),
            Self::EmptyWithAntinode => write!(f, "#"),
            Self::AntennaWithAntinode(c) => write!(f, "{}", c),
        }
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(Location::from_char);
        let mut clone = grid.clone();

        for (p, p2) in pairs(&grid) {
            let antinode_location = p - (p2 - p);

            if clone.in_bounds(&antinode_location) {
                let a = clone.get(&antinode_location).unwrap();
                let new_location = match a {
                    Location::Empty => Location::EmptyWithAntinode,
                    Location::EmptyWithAntinode => Location::EmptyWithAntinode,
                    Location::Antenna(c) => Location::AntennaWithAntinode(*c),
                    Location::AntennaWithAntinode(c) => Location::AntennaWithAntinode(*c),
                };
                clone.set(&antinode_location, new_location);
            }
        }

        let result = clone
            .iter()
            .filter(|(_, l)| match **l {
                Location::Empty => false,
                Location::EmptyWithAntinode => true,
                Location::Antenna(_) => false,
                Location::AntennaWithAntinode(_) => true,
            })
            .count();

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
    }

    fn expected_output_one(&self) -> &str {
        "14"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(Location::from_char);
        let mut clone = grid.clone();

        for (p, p2) in pairs(&grid) {
            let diff = p2 - p;
            let mut antinode_location = p;
            while clone.in_bounds(&antinode_location) {
                let a = clone.get(&antinode_location).unwrap();
                let new_location = match a {
                    Location::Empty => Location::EmptyWithAntinode,
                    Location::EmptyWithAntinode => Location::EmptyWithAntinode,
                    Location::Antenna(c) => Location::AntennaWithAntinode(*c),
                    Location::AntennaWithAntinode(c) => Location::AntennaWithAntinode(*c),
                };
                clone.set(&antinode_location, new_location);
                antinode_location = antinode_location - diff;
            }
        }

        let result = clone
            .iter()
            .filter(|(_, l)| match **l {
                Location::Empty => false,
                Location::EmptyWithAntinode => true,
                Location::Antenna(_) => false,
                Location::AntennaWithAntinode(_) => true,
            })
            .count();

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
    }

    fn expected_output_two(&self) -> &str {
        "34"
    }
}

fn pairs(grid: &Grid2d<Location>) -> Vec<(VecI2, VecI2)> {
    let antennas = grid
        .iter()
        .filter(|(_, l)| l.is_antenna())
        .map(|(l, p)| (l, *p))
        .collect::<Vec<_>>();

    antennas
        .iter()
        .cartesian_product(antennas.iter())
        .filter(|((l, p), (l2, p2))| *l != *l2 && p.is_same_antenna_type(p2))
        .map(|((l, _), (l2, _))| (*l, *l2))
        .collect()
}
