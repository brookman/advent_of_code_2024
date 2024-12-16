use std::{collections::HashSet, fmt::Display, hash::Hash};

use crate::common::*;
use pathfinding::prelude::{astar, astar_bag_collect};

pub struct S;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Entity {
    Empty,
    Start,
    End,
    Wall,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Empty => write!(f, "."),
            Entity::Start => write!(f, "S"),
            Entity::End => write!(f, "E"),
            Entity::Wall => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    pub pos: VecI2,
    pub dir: Direction,
}

impl Pos {
    fn distance(&self, other: &VecI2) -> u32 {
        let manhattan_distance = self.pos.0.abs_diff(other.0) + self.pos.1.abs_diff(other.1);

        let rot_distance = match self.dir {
            Direction::Left => {
                if self.pos.0 >= other.0 && self.pos.1 == other.1 {
                    0
                } else if self.pos.0 == other.0 {
                    1
                } else {
                    2
                }
            }
            Direction::Right => {
                if self.pos.0 <= other.0 && self.pos.1 == other.1 {
                    0
                } else if self.pos.0 == other.0 {
                    1
                } else {
                    2
                }
            }
            Direction::Up => {
                if self.pos.0 == other.0 && self.pos.1 >= other.1 {
                    0
                } else if self.pos.1 == other.1 {
                    1
                } else {
                    2
                }
            }
            Direction::Down => {
                if self.pos.0 == other.0 && self.pos.1 <= other.1 {
                    0
                } else if self.pos.1 == other.1 {
                    1
                } else {
                    2
                }
            }
        };

        manhattan_distance + rot_distance * 1000
    }

    fn successors(&self, grid: &Grid2d<Entity>) -> Vec<(Pos, u32)> {
        let mut succ = vec![];
        match self.dir {
            Direction::Left | Direction::Right => {
                succ.push((
                    Pos {
                        pos: self.pos,
                        dir: Direction::Up,
                    },
                    1000,
                ));
                succ.push((
                    Pos {
                        pos: self.pos,
                        dir: Direction::Down,
                    },
                    1000,
                ));
            }
            Direction::Up | Direction::Down => {
                succ.push((
                    Pos {
                        pos: self.pos,
                        dir: Direction::Left,
                    },
                    1000,
                ));
                succ.push((
                    Pos {
                        pos: self.pos,
                        dir: Direction::Right,
                    },
                    1000,
                ))
            }
        }
        let new_pos = self.pos.dir(&self.dir);
        if let Some(new_pos_entity) = grid.get(&new_pos) {
            if new_pos_entity != &Entity::Wall {
                succ.push((
                    Pos {
                        pos: new_pos,
                        dir: self.dir,
                    },
                    1,
                ));
            }
        }
        succ
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| match c {
            '.' => Entity::Empty,
            '#' => Entity::Wall,
            'S' => Entity::Start,
            'E' => Entity::End,
            _ => panic!("Unknown char: {}", c),
        });
        let start = grid.find_first(|e| matches!(e, Entity::Start)).unwrap().0;
        let end = grid.find_first(|e| matches!(e, Entity::End)).unwrap().0;

        let start = Pos {
            pos: start,
            dir: Direction::Right,
        };

        // println!("{:?}", grid);
        // println!("{:?}", start);
        // println!("{:?}", end);

        let result = astar(
            &start,
            |p| p.successors(&grid),
            |p| p.distance(&end),
            |p| p.pos == end,
        )
        .unwrap()
        .1;

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#
    }

    fn expected_output_one(&self) -> &str {
        "7036"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| match c {
            '.' => Entity::Empty,
            '#' => Entity::Wall,
            'S' => Entity::Start,
            'E' => Entity::End,
            _ => panic!("Unknown char: {}", c),
        });
        let start = grid.find_first(|e| matches!(e, Entity::Start)).unwrap().0;
        let end = grid.find_first(|e| matches!(e, Entity::End)).unwrap().0;

        let start = Pos {
            pos: start,
            dir: Direction::Right,
        };

        let mut best_path_tiles = HashSet::new();

        let result = astar_bag_collect(
            &start,
            |p| p.successors(&grid),
            |p| p.distance(&end),
            |p| p.pos == end,
        )
        .unwrap()
        .0;

        for path in result.iter() {
            for pos in path.iter() {
                best_path_tiles.insert(pos.pos);
            }
        }

        best_path_tiles.len().to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#
    }

    fn expected_output_two(&self) -> &str {
        "45"
    }
}
