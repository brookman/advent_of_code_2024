use std::fmt::Display;

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

impl Display for Grid2d<Location> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_row = 0;
        for (_, row, l) in self.iter() {
            if row != last_row {
                writeln!(f).unwrap();
            }
            last_row = row;
            write!(f, "{}", l).unwrap();
        }
        Ok(())
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(Location::from_char);
        let mut clone = grid.clone();

        for (x, y, l) in grid.iter() {
            for (x2, y2, l2) in grid.iter() {
                if (x == x2 && y == y2) || !l.is_same_antenna_type(l2) {
                    continue;
                }

                let antinode_location = (x - (x2 - x), y - (y2 - y));

                if clone.in_bounds(antinode_location) {
                    let a = clone.get(antinode_location).unwrap();
                    let new_location = match a {
                        Location::Empty => Location::EmptyWithAntinode,
                        Location::EmptyWithAntinode => Location::EmptyWithAntinode,
                        Location::Antenna(c) => Location::AntennaWithAntinode(*c),
                        Location::AntennaWithAntinode(c) => Location::AntennaWithAntinode(*c),
                    };
                    clone.set(antinode_location, new_location);
                }
            }
        }

        let result = clone
            .iter()
            .filter(|(_, _, l)| match **l {
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

        println!("{}", clone);

        for (x, y, l) in grid.iter() {
            for (x2, y2, l2) in grid.iter() {
                if (x == x2 && y == y2) || !l.is_same_antenna_type(l2) {
                    continue;
                }

                println!("{} {} {} {}", x, y, x2, y2);
                let diff = (x2 - x, y2 - y);
                let mut antinode_location = (x, y);
                while clone.in_bounds(antinode_location) {
                    let a = clone.get(antinode_location).unwrap();
                    let new_location = match a {
                        Location::Empty => Location::EmptyWithAntinode,
                        Location::EmptyWithAntinode => Location::EmptyWithAntinode,
                        Location::Antenna(c) => Location::AntennaWithAntinode(*c),
                        Location::AntennaWithAntinode(c) => Location::AntennaWithAntinode(*c),
                    };
                    clone.set(antinode_location, new_location);
                    antinode_location =
                        (antinode_location.0 - diff.0, antinode_location.1 - diff.1);
                }
            }
        }

        println!("{}", clone);

        let result = clone
            .iter()
            .filter(|(_, _, l)| match **l {
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
