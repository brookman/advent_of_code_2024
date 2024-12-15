use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::common::*;

const TEST: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

pub struct S;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entity {
    Empty,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Empty => write!(f, "."),
            Entity::Robot => write!(f, "@"),
            Entity::Box => write!(f, "O"),
            Entity::BoxLeft => write!(f, "["),
            Entity::BoxRight => write!(f, "]"),
            Entity::Wall => write!(f, "#"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
        }
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        solve(&input.lines, |c| c.to_string()).to_string()
    }

    fn test_input_one(&self) -> &str {
        TEST
    }

    fn expected_output_one(&self) -> &str {
        "10092"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        solve(&input.lines, |c| {
            match c {
                '.' => "..",
                '@' => "@.",
                'O' => "[]",
                '#' => "##",
                _ => panic!("Unknown char: {}", c),
            }
            .to_string()
        })
        .to_string()
    }

    fn test_input_two(&self) -> &str {
        TEST
    }

    fn expected_output_two(&self) -> &str {
        "9021"
    }
}

fn solve(lines: &[String], line_modifier: fn(char) -> String) -> u32 {
    let mut grid_lines = vec![];
    let mut move_lines = vec![];
    let mut iter = lines.iter();
    let mut next_line = iter.next();
    while next_line.is_some() {
        let line = next_line.unwrap();
        next_line = iter.next();
        if line.is_empty() {
            break;
        }
        let modified_line = line.chars().map(line_modifier).collect();
        grid_lines.push(modified_line);
    }
    while next_line.is_some() {
        let line = next_line.unwrap();
        next_line = iter.next();
        move_lines.push(line.to_string());
    }

    let mut grid = parse_grid(&grid_lines);
    let moves = parse_moves(&move_lines);

    let mut robot_pos = grid.find_first(|e| matches!(e, Entity::Robot)).unwrap().0;

    for direction in moves {
        if can_move(&grid, robot_pos, &direction) {
            move_it(&mut grid, robot_pos, &direction);
            robot_pos = match direction {
                Direction::Left => robot_pos.left(),
                Direction::Right => robot_pos.right(),
                Direction::Up => robot_pos.up(),
                Direction::Down => robot_pos.down(),
            };
        }
    }

    count_boxes(&grid)
}

fn parse_grid(grid_lines: &[String]) -> Grid2d<Entity> {
    Grid2d::from_lines(grid_lines, |c| match c {
        '.' => Entity::Empty,
        '@' => Entity::Robot,
        'O' => Entity::Box,
        '[' => Entity::BoxLeft,
        ']' => Entity::BoxRight,
        '#' => Entity::Wall,
        _ => panic!("Unknown char: {}", c),
    })
}

fn parse_moves(move_lines: &[String]) -> Vec<Direction> {
    move_lines
        .iter()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown char: {}", c),
        })
        .collect()
}

fn can_move(grid: &Grid2d<Entity>, pos: VecI2, direction: &Direction) -> bool {
    let new_pos = match direction {
        Direction::Left => pos.left(),
        Direction::Right => pos.right(),
        Direction::Up => pos.up(),
        Direction::Down => pos.down(),
    };

    let target = grid.get(&new_pos);
    if target.is_none() {
        return false;
    }
    let target = target.unwrap();

    match target {
        Entity::Empty => true,
        Entity::Robot => panic!("There must only be one robot."),
        Entity::Box => can_move(grid, new_pos, direction),
        Entity::BoxLeft => {
            let is_horizontal = matches!(direction, Direction::Left | Direction::Right);
            if is_horizontal {
                can_move(grid, new_pos, direction)
            } else {
                can_move(grid, new_pos, direction) && can_move(grid, new_pos.right(), direction)
            }
        }
        Entity::BoxRight => {
            let is_horizontal = matches!(direction, Direction::Left | Direction::Right);
            if is_horizontal {
                can_move(grid, new_pos, direction)
            } else {
                can_move(grid, new_pos, direction) && can_move(grid, new_pos.left(), direction)
            }
        }
        Entity::Wall => false,
    }
}

fn move_it(grid: &mut Grid2d<Entity>, pos: VecI2, direction: &Direction) {
    let current = grid.get(&pos).unwrap().clone();

    let new_pos = match direction {
        Direction::Left => pos.left(),
        Direction::Right => pos.right(),
        Direction::Up => pos.up(),
        Direction::Down => pos.down(),
    };

    let target = grid.get(&new_pos).unwrap().clone();

    match target {
        Entity::Empty => {
            grid.set(&pos, Entity::Empty);
            grid.set(&new_pos, current);
        }
        Entity::Robot => panic!("There must only be one robot."),
        Entity::Box => {
            move_it(grid, new_pos, direction);
            grid.set(&pos, Entity::Empty);
            grid.set(&new_pos, current);
        }
        Entity::BoxLeft => {
            let is_horizontal = matches!(direction, Direction::Left | Direction::Right);
            if is_horizontal {
                move_it(grid, new_pos, direction);
            } else {
                move_it(grid, new_pos, direction);
                move_it(grid, new_pos.right(), direction);
            }
            grid.set(&pos, Entity::Empty);
            grid.set(&new_pos, current);
        }
        Entity::BoxRight => {
            let is_horizontal = matches!(direction, Direction::Left | Direction::Right);
            if is_horizontal {
                move_it(grid, new_pos, direction);
            } else {
                move_it(grid, new_pos, direction);
                move_it(grid, new_pos.left(), direction);
            }
            grid.set(&pos, Entity::Empty);
            grid.set(&new_pos, current);
        }
        Entity::Wall => {}
    }
}

fn count_boxes(grid: &Grid2d<Entity>) -> u32 {
    grid.iter()
        .filter(|e| matches!(e.1, Entity::Box | Entity::BoxLeft))
        .map(|e| e.0 .1 as u32 * 100 + e.0 .0 as u32)
        .sum()
}
