use std::collections::HashSet;

use crate::common::*;

pub struct S;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trail {
    pub trail: Vec<VecI2>,
    pub visited: HashSet<VecI2>,
}

impl Trail {
    pub fn already_visited(&self, pos: &VecI2) -> bool {
        self.visited.contains(pos)
    }

    pub fn visit(&mut self, pos: &VecI2) {
        self.trail.push(*pos);
        self.visited.insert(*pos);
    }
}

fn get_trailheads(pos: VecI2, grid: &Grid2d<u8>, visited: &mut HashSet<VecI2>) -> u32 {
    //let mut visited = visited.clone();
    let value = grid.get(&pos);
    //println!("{:?} {:?}", pos, value);
    if value.is_none() || visited.contains(&pos) {
        return 0;
    }
    let value = value.unwrap();

    visited.insert(pos);
    //println!("{:?}", visited);

    if value == &9 {
        return 1;
    }

    let mut sum = 0;

    let left = pos.left();
    if grid.in_bounds(&left) && *grid.get(&left).unwrap() == value + 1 {
        sum += get_trailheads(left, grid, visited);
    }
    let up = pos.up();
    if grid.in_bounds(&up) && *grid.get(&up).unwrap() == value + 1 {
        sum += get_trailheads(up, grid, visited);
    }
    let right = pos.right();
    if grid.in_bounds(&right) && *grid.get(&right).unwrap() == value + 1 {
        sum += get_trailheads(right, grid, visited);
    }
    let down = pos.down();
    if grid.in_bounds(&down) && *grid.get(&down).unwrap() == value + 1 {
        sum += get_trailheads(down, grid, visited);
    }

    sum
}

fn get_trailheads2(pos: VecI2, grid: &Grid2d<u8>, trail: &mut Trail) -> Vec<Trail> {
    let mut trail = trail.clone();
    let value = grid.get(&pos);
    //println!("{:?} {:?}", pos, value);
    if value.is_none() || trail.already_visited(&pos) {
        return vec![];
    }
    let value = value.unwrap();

    trail.visit(&pos);
    //println!("{:?}", visited);

    if value == &9 {
        return vec![trail];
    }

    let mut trails = vec![];

    let left = pos.left();
    if grid.in_bounds(&left) && *grid.get(&left).unwrap() == value + 1 {
        trails.extend(get_trailheads2(left, grid, &mut trail));
    }
    let up = pos.up();
    if grid.in_bounds(&up) && *grid.get(&up).unwrap() == value + 1 {
        trails.extend(get_trailheads2(up, grid, &mut trail));
    }
    let right = pos.right();
    if grid.in_bounds(&right) && *grid.get(&right).unwrap() == value + 1 {
        trails.extend(get_trailheads2(right, grid, &mut trail));
    }
    let down = pos.down();
    if grid.in_bounds(&down) && *grid.get(&down).unwrap() == value + 1 {
        trails.extend(get_trailheads2(down, grid, &mut trail));
    }

    trails
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| c.to_digit(10).unwrap() as u8);

        let start_positions = grid
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(p, _)| p)
            .collect::<Vec<_>>();

        let mut result = 0;
        for start_pos in start_positions {
            let mut visited = HashSet::new();
            let trailheads = get_trailheads(start_pos, &grid, &mut visited);
            result += trailheads;
        }

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#
    }

    fn expected_output_one(&self) -> &str {
        "36"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| c.to_digit(10).unwrap() as u8);

        let start_positions = grid
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(p, _)| p)
            .collect::<Vec<_>>();

        let mut result = 0;
        for start_pos in start_positions {
            let mut trail = Trail {
                trail: vec![],
                visited: HashSet::new(),
            };
            let trailheads = get_trailheads2(start_pos, &grid, &mut trail);
            result += trailheads.len();
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#
    }

    fn expected_output_two(&self) -> &str {
        "81"
    }
}
