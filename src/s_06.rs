use ndarray::Array2;
use std::{collections::HashSet, fmt::Display};

use crate::common::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    pos: (usize, usize),
    direction: Direction,
}

impl Display for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2d {
    width: usize,
    height: usize,
    map: Array2<MapTile>,
    guard: Option<Guard>,
    visited: HashSet<(usize, usize)>,
    visited_with_dir: HashSet<(usize, usize, Direction)>,
}

impl Grid2d {
    fn new(string: &str) -> Self {
        let lines: Vec<String> = string.lines().map(|s| s.to_string()).collect();
        let width = lines[0].len();
        let height = lines.len();
        let shape = (height, width);

        let chars = lines
            .iter()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        let map_tiles = chars
            .clone()
            .into_iter()
            .map(MapTile::from_char)
            .collect::<Vec<_>>();

        let map = Array2::from_shape_vec(shape, map_tiles).expect("Shape mismatch");

        let guard_up = chars.iter().position(|c| *c == '^').map(|i| {
            let x = i % width;
            let y = i / width;
            let pos = (x, y);
            Guard {
                pos,
                direction: Direction::Up,
            }
        });

        let guard_right = chars.iter().position(|c| *c == '>').map(|i| {
            let x = i % width;
            let y = i / width;
            let pos = (x, y);
            Guard {
                pos,
                direction: Direction::Right,
            }
        });

        let guard_down = chars.iter().position(|c| *c == 'v').map(|i| {
            let x = i % width;
            let y = i / width;
            let pos = (x, y);
            Guard {
                pos,
                direction: Direction::Down,
            }
        });

        let guard_left = chars.iter().position(|c| *c == '<').map(|i| {
            let x = i % width;
            let y = i / width;
            let pos = (x, y);
            Guard {
                pos,
                direction: Direction::Left,
            }
        });

        let guard = if guard_up.is_some() {
            guard_up.unwrap()
        } else if guard_right.is_some() {
            guard_right.unwrap()
        } else if guard_down.is_some() {
            guard_down.unwrap()
        } else if guard_left.is_some() {
            guard_left.unwrap()
        } else {
            panic!("No guard found")
        };

        Self {
            width,
            height,
            map,
            guard: Some(guard),
            visited: HashSet::new(),
            visited_with_dir: HashSet::new(),
        }
    }

    fn pos_from_index(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn index_from_pos(&self, pos: (usize, usize)) -> usize {
        pos.0 + pos.1 * self.width
    }

    fn get_pos(&self, pos: (usize, usize)) -> Option<&MapTile> {
        self.map.get((pos.1, pos.0))
    }

    fn set_pos(&mut self, pos: (usize, usize), value: MapTile) {
        if let Some(a) = self.map.get_mut((pos.1, pos.0)) {
            *a = value;
        }
    }

    fn get_index(&self, index: usize) -> Option<&MapTile> {
        let pos = self.pos_from_index(index);
        self.get_pos(pos)
    }

    fn set_index(&mut self, index: usize, value: MapTile) {
        let pos = self.pos_from_index(index);
        if let Some(a) = self.map.get_mut((pos.1, pos.0)) {
            *a = value;
        }
    }

    fn print(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.guard.is_some() && self.guard.unwrap().pos == (x, y) {
                    result.push_str(match self.guard.unwrap().direction {
                        Direction::Up => "^",
                        Direction::Right => ">",
                        Direction::Down => "v",
                        Direction::Left => "<",
                    });
                } else if self.visited.contains(&(x, y)) {
                    result.push('X');
                } else {
                    let tile = self.get_pos((x, y)).unwrap();
                    if tile == &MapTile::Empty {
                        result.push('.');
                    } else {
                        result.push('#');
                    }
                }
            }
            result.push('\n');
        }
        result
    }

    fn solve(&mut self) -> Option<usize> {
        while self.guard.is_some() {
            let guard = self.guard.unwrap();
            self.visited.insert(guard.pos);
            let pos_dir = (guard.pos.0, guard.pos.1, guard.direction);
            if self.visited_with_dir.contains(&pos_dir) {
                println!("loop detected");
                return None;
            }
            self.visited_with_dir.insert(pos_dir);

            let next_pos = match guard.direction {
                Direction::Up => (guard.pos.0 as i32, guard.pos.1 as i32 - 1),
                Direction::Right => (guard.pos.0 as i32 + 1, guard.pos.1 as i32),
                Direction::Down => (guard.pos.0 as i32, guard.pos.1 as i32 + 1),
                Direction::Left => (guard.pos.0 as i32 - 1, guard.pos.1 as i32),
            };
            let is_inside = next_pos.0 >= 0
                && next_pos.0 < self.width as i32
                && next_pos.1 >= 0
                && next_pos.1 < self.height as i32;

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            if is_inside {
                if self.get_pos(next_pos).unwrap() == &MapTile::Empty {
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

impl Display for Grid2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let mut grid = Grid2d::new(&input.input);

        let result = grid.solve().unwrap();
        println!("{}", grid);

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
        let mut grid = Grid2d::new(&input.input);

        let mut result = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let pos = grid.get_pos((x, y)).unwrap();
                if pos == &MapTile::Empty && grid.guard.unwrap().pos != (x, y) {
                    let mut cloned = grid.clone();
                    cloned.set_pos((x, y), MapTile::Obstacle);
                    if cloned.solve().is_none() {
                        result += 1;
                    }
                }
            }
        }
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
