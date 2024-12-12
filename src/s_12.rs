use std::time::Instant;

use rustc_hash::FxHashSet;

use crate::{common::*, geometry::*};

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

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let grid = input.grid2d(|c| c);
        let regions = get_regions(&grid);

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
        let regions = get_regions(&grid);

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

        let mut result = 0;

        for region in regions {
            let bounds = region
                .iter()
                .fold(None::<Bounds2>, |acc, pos| {
                    acc.extend(&Some(Bounds2::from_point(*pos)))
                })
                .unwrap();

            let mut wall_count = 0;

            for y in bounds.min().1..=bounds.max().1 {
                let mut previous_up: bool = false;
                let mut previous_down: bool = false;
                for x in bounds.min().0..=bounds.max().0 {
                    let pos = VecI2(x, y);

                    if !region.contains(&pos) {
                        previous_up = false;
                        previous_down = false;
                        continue;
                    }
                    let cell = cell_grid.get(&pos).unwrap();
                    if cell.up_border && !previous_up {
                        wall_count += 1;
                    }

                    if cell.down_border && !previous_down {
                        wall_count += 1;
                    }

                    previous_up = cell.up_border;
                    previous_down = cell.down_border;
                }
            }

            for x in bounds.min().0..=bounds.max().0 {
                let mut previous_left: bool = false;
                let mut previous_right: bool = false;
                for y in bounds.min().1..=bounds.max().1 {
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

fn get_regions(grid: &Grid2d<char>) -> Vec<FxHashSet<VecI2>> {
    let mut regions: Vec<FxHashSet<VecI2>> = vec![];
    let mut already_visited: FxHashSet<VecI2> = FxHashSet::default();

    for (pos, _) in grid.iter() {
        if !already_visited.contains(&pos) {
            let mut new_region = FxHashSet::default();
            visit(grid, &pos, &mut new_region);
            already_visited.extend(new_region.iter());
            regions.push(new_region);
        }
    }
    regions
}

fn visit(grid: &Grid2d<char>, pos: &VecI2, region: &mut FxHashSet<VecI2>) {
    region.insert(*pos);

    let current = grid.get(pos).unwrap();

    let new_pos = pos.left();
    if let Some(value) = grid.get(&new_pos) {
        if value == current && !region.contains(&new_pos) {
            visit(grid, &new_pos, region);
        }
    }

    let new_pos = pos.right();
    if let Some(value) = grid.get(&new_pos) {
        if value == current && !region.contains(&new_pos) {
            visit(grid, &new_pos, region);
        }
    }

    let new_pos = pos.up();
    if let Some(value) = grid.get(&new_pos) {
        if value == current && !region.contains(&new_pos) {
            visit(grid, &new_pos, region);
        }
    }

    let new_pos = pos.down();
    if let Some(value) = grid.get(&new_pos) {
        if value == current && !region.contains(&new_pos) {
            visit(grid, &new_pos, region);
        }
    }
}
