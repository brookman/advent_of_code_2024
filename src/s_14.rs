use std::cmp::Ordering;

use crate::common::*;
use lazy_static::lazy_static;
use regex::Regex;

pub struct S;

lazy_static! {
    static ref LINE: Regex = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
}

fn mod_neg(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let width = 101;
        let height = 103;

        let get_numbers = |regex: &Regex, string: &String| -> (VecI2, VecI2) {
            let cap = regex.captures_iter(string).next().unwrap();
            (
                VecI2(
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                VecI2(
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                ),
            )
        };

        let mut left_top = 0;
        let mut right_top = 0;
        let mut left_bottom = 0;
        let mut right_bottom = 0;

        for line in input.lines.iter() {
            let (p, v) = get_numbers(&LINE, line);
            let end = p + v * 100;
            let end = VecI2(mod_neg(end.0, width), mod_neg(end.1, height));

            match (end.0.cmp(&(width / 2)), end.1.cmp(&(height / 2))) {
                (Ordering::Less, Ordering::Less) => left_top += 1,
                (Ordering::Less, Ordering::Greater) => left_bottom += 1,
                (Ordering::Greater, Ordering::Less) => right_top += 1,
                (Ordering::Greater, Ordering::Greater) => right_bottom += 1,
                _ => {}
            }
        }

        let result = left_top * right_top * left_bottom * right_bottom;

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#
    }

    fn expected_output_one(&self) -> &str {
        "21"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let width: i32 = 101;
        let height: i32 = 103;

        let get_numbers = |regex: &Regex, string: &String| -> (VecI2, VecI2) {
            let cap = regex.captures_iter(string).next().unwrap();
            (
                VecI2(
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                VecI2(
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                ),
            )
        };

        const GRID_W: usize = 3;
        const GRID_H: usize = 3;

        for steps in 0..=10000 {
            let mut buckets = [0; GRID_H * GRID_W];

            for line in input.lines.iter() {
                let (p, v) = get_numbers(&LINE, line);
                let end = p + v * steps;
                let end = VecI2(mod_neg(end.0, width), mod_neg(end.1, height));

                let q1 = (end.0 as f32 / width as f32 * GRID_W as f32).clamp(0.0, GRID_W as f32)
                    as usize;
                let q2 = (end.1 as f32 / height as f32 * GRID_H as f32).clamp(0.0, GRID_H as f32)
                    as usize;
                buckets[(q2 * GRID_W + q1) as usize] += 1;
            }

            let weighted_max = weighted_max(&buckets);

            if weighted_max > 0.5 {
                return steps.to_string();
            }
        }

        "".to_string()
    }

    fn test_input_two(&self) -> &str {
        ""
    }

    fn expected_output_two(&self) -> &str {
        ""
    }
}

fn weighted_max(input: &[i32]) -> f32 {
    let max_value = *input.iter().max().unwrap();
    let total: i32 = input.iter().sum();
    max_value as f32 / total as f32
}
