use std::collections::{HashMap, HashSet};

use crate::common::*;

pub struct S;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub pos: VecI2,
    pub value: char,
    pub left_border: bool,
    pub right_border: bool,
    pub up_border: bool,
    pub down_border: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounds {
    min: Option<VecI2>,
    max: Option<VecI2>,
}

impl Bounds {
    pub fn new() -> Self {
        Bounds {
            min: None,
            max: None,
        }
    }
    pub fn from_vec2i(vec: &VecI2) -> Self {
        Bounds {
            min: Some(*vec),
            max: Some(*vec),
        }
    }

    pub fn extend(&self, other: Bounds) -> Bounds {
        let min = if self.min.is_none() {
            other.min
        } else if other.min.is_none() {
            self.min
        } else {
            Some(self.min.unwrap().min(&other.min.unwrap()))
        };

        let max = if self.max.is_none() {
            other.max
        } else if other.max.is_none() {
            self.max
        } else {
            Some(self.max.unwrap().max(&other.max.unwrap()))
        };

        Bounds { min, max }
    }
}

impl Cell {
    pub fn has_any_border(&self) -> bool {
        self.left_border || self.right_border || self.up_border || self.down_border
    }

    pub fn same_type_as(&self, other: &Option<Cell>) -> bool {
        if let Some(other) = other {
            other.value == self.value
        } else {
            false
        }
    }

    pub fn left(&self, cells: &Grid2d<Cell>) -> Option<Cell> {
        let left = self.pos.left();
        cells.get(&left).map(|c| c.clone())
    }

    pub fn right(&self, cells: &Grid2d<Cell>) -> Option<Cell> {
        let right = self.pos.right();
        cells.get(&right).map(|c| c.clone())
    }

    pub fn up(&self, cells: &Grid2d<Cell>) -> Option<Cell> {
        let up = self.pos.up();
        cells.get(&up).map(|c| c.clone())
    }

    pub fn down(&self, cells: &Grid2d<Cell>) -> Option<Cell> {
        let down = self.pos.down();
        cells.get(&down).map(|c| c.clone())
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| c);
        let mut regions: Vec<HashSet<VecI2>> = vec![];
        let mut already_visited = HashSet::new();

        for (pos, _) in grid.iter() {
            if !already_visited.contains(&pos) {
                already_visited.insert(pos);
                let mut new_region = HashSet::new();
                visit(&grid, &pos, &mut new_region);
                already_visited.extend(new_region.iter());
                regions.push(new_region);
            }
        }

        let mut result = 0;

        for region in regions {
            let mut perimeter = 0;
            for pos in region.iter() {
                let current = grid.get(pos).unwrap();

                let left = grid.get(&pos.left());
                let right = grid.get(&pos.right());
                let up = grid.get(&pos.up());
                let down = grid.get(&pos.down());

                if left.is_none() || left.unwrap() != current {
                    perimeter += 1;
                }
                if right.is_none() || right.unwrap() != current {
                    perimeter += 1;
                }
                if up.is_none() || up.unwrap() != current {
                    perimeter += 1;
                }
                if down.is_none() || down.unwrap() != current {
                    perimeter += 1;
                }
            }

            result += region.len() * perimeter;
        }

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#
    }

    fn expected_output_one(&self) -> &str {
        "1930"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| c);
        let mut regions: Vec<HashSet<VecI2>> = vec![];
        let mut index_lookup = HashMap::<VecI2, usize>::new();
        let mut already_visited = HashSet::new();

        for (pos, _) in grid.iter() {
            if !already_visited.contains(&pos) {
                already_visited.insert(pos);
                let mut new_region = HashSet::new();
                visit(&grid, &pos, &mut new_region);
                already_visited.extend(new_region.iter());

                let index = regions.len();
                index_lookup.insert(pos, index);
                regions.push(new_region);
            }
        }

        let cells = grid
            .iter()
            .map(|(pos, c)| {
                let left = grid.get(&pos.left());
                let right = grid.get(&pos.right());
                let up = grid.get(&pos.up());
                let down = grid.get(&pos.down());

                Cell {
                    pos,
                    value: *c,
                    left_border: left.is_none() || left.unwrap() != c,
                    right_border: right.is_none() || right.unwrap() != c,
                    up_border: up.is_none() || up.unwrap() != c,
                    down_border: down.is_none() || down.unwrap() != c,
                }
            })
            .collect::<Vec<_>>();
        let cell_grid = Grid2d::new(grid.width, grid.height, cells);

        //println!("{:?}", border_grid);

        let mut result = 0;

        for region in regions {
            let bounds = region.iter().fold(Bounds::new(), |acc, pos| {
                acc.extend(Bounds::from_vec2i(pos))
            });

            println!("region bounds {:?}", bounds);

            let mut wall_count = 0;

            for y in bounds.min.map(|b| b.1).unwrap_or(0)..=bounds.max.map(|b| b.1).unwrap_or(0) {
                let mut previous_up: bool = false;
                let mut previous_down: bool = false;
                for x in bounds.min.map(|b| b.0).unwrap_or(0)..=bounds.max.map(|b| b.0).unwrap_or(0)
                {
                    let pos = VecI2(x, y);
                    println!("checking {:?}", pos);

                    if !region.contains(&pos) {
                        previous_up = false;
                        previous_down = false;
                        println!("not in region");
                        continue;
                    }
                    let cell = cell_grid.get(&pos).unwrap();
                    if cell.up_border && !previous_up {
                        println!("up plus");
                        wall_count += 1;
                    }

                    if cell.down_border && !previous_down {
                        println!("down plus");
                        wall_count += 1;
                    }

                    previous_up = cell.up_border;
                    previous_down = cell.down_border;
                }
            }

            for x in bounds.min.map(|b| b.0).unwrap_or(0)..=bounds.max.map(|b| b.0).unwrap_or(0) {
                let mut previous_left: bool = false;
                let mut previous_right: bool = false;
                for y in bounds.min.map(|b| b.1).unwrap_or(0)..=bounds.max.map(|b| b.1).unwrap_or(0)
                {
                    let pos = VecI2(x, y);

                    if !region.contains(&pos) {
                        previous_left = false;
                        previous_right = false;
                        continue;
                    }
                    let cell = cell_grid.get(&pos).unwrap();
                    if cell.left_border && !previous_left {
                        wall_count += 1;
                    }

                    if cell.right_border && !previous_right {
                        wall_count += 1;
                    }

                    previous_left = cell.left_border;
                    previous_right = cell.right_border;
                }
            }

            println!("{} {}", region.len(), wall_count);
            result += region.len() * wall_count;
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#
    }

    fn expected_output_two(&self) -> &str {
        "1206"
    }
}

fn visit(grid: &Grid2d<char>, pos: &VecI2, region: &mut HashSet<VecI2>) {
    if region.contains(pos) {
        return;
    }
    region.insert(*pos);

    let current = grid.get(pos).unwrap();

    if let Some(left) = grid.get(&pos.left()) {
        if left == current {
            visit(grid, &pos.left(), region);
        }
    }

    if let Some(right) = grid.get(&pos.right()) {
        if right == current {
            visit(grid, &pos.right(), region);
        }
    }

    if let Some(up) = grid.get(&pos.up()) {
        if up == current {
            visit(grid, &pos.up(), region);
        }
    }

    if let Some(down) = grid.get(&pos.down()) {
        if down == current {
            visit(grid, &pos.down(), region);
        }
    }
}
