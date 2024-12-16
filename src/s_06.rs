use rayon::prelude::*;

use crate::common::*;
use std::{collections::HashSet, fmt::Display};

pub struct S;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Empty = 0,
    Obstacle = 1,
}

impl MapTile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Obstacle,
            _ => Self::Empty,
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: VecI2,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    guard: Option<Guard>,
    additional_obstacle: Option<VecI2>,
    visited: HashSet<VecI2>,
    visited_with_dir: HashSet<Guard>,
}

impl Puzzle {
    fn new(input: &PuzzleInput, width: usize) -> Self {
        let guard = input
            .input
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .filter_map(|(i, c)| Direction::from_char(c).map(|direction| (i, direction)))
            .map(|(i, direction)| {
                let pos = VecI2((i % width) as i32, (i / width) as i32);
                Guard { pos, direction }
            })
            .next();

        if guard.is_none() {
            panic!("No guard found");
        }

        Self {
            guard,
            additional_obstacle: None,
            visited: HashSet::new(),
            visited_with_dir: HashSet::new(),
        }
    }

    fn solve(&mut self, grid: &Grid2d<MapTile>) -> Option<usize> {
        while self.guard.is_some() {
            let guard = self.guard.unwrap();
            self.visited.insert(guard.pos);

            if self.visited_with_dir.contains(&guard) {
                return None;
            }
            self.visited_with_dir.insert(guard);

            let next_pos = guard.pos.dir(&guard.direction);

            if grid.in_bounds(&next_pos) {
                if grid.get(&next_pos).unwrap() == &MapTile::Empty
                    && self.additional_obstacle != Some(next_pos)
                {
                    self.guard = Some(Guard {
                        pos: next_pos,
                        direction: guard.direction,
                    });
                } else {
                    self.guard = Some(Guard {
                        pos: guard.pos,
                        direction: match guard.direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        },
                    });
                }
            } else {
                self.guard = None;
            }
        }
        //println!("{}", grid);
        Some(self.visited.len())
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(MapTile::from_char);
        let mut puzzle = Puzzle::new(input, grid.width);
        let result = puzzle.solve(&grid).unwrap();
        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
    }

    fn expected_output_one(&self) -> &str {
        "41"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(MapTile::from_char);
        let puzzle = Puzzle::new(input, grid.width);
        let mut solved_puzzle = puzzle.clone();
        solved_puzzle.solve(&grid).unwrap();

        let guard_pos = puzzle.guard.unwrap().pos;
        let candiates = solved_puzzle
            .visited
            .into_iter()
            .filter(|p| p != &guard_pos)
            .collect::<Vec<_>>();

        let result: u32 = candiates
            .par_iter()
            .map(|pos| {
                let mut cloned = puzzle.clone();
                cloned.additional_obstacle = Some(*pos);
                if cloned.solve(&grid).is_none() {
                    1
                } else {
                    0
                }
            })
            .sum();
        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
    }

    fn expected_output_two(&self) -> &str {
        "6"
    }
}
