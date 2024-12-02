use crate::common::*;

pub struct S {}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> Option<String> {
        let mut safe_lines = 0;
        for line in &input.lines {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if is_safe(&levels) {
                safe_lines += 1;
            }
        }
        Some(format!("{}", safe_lines))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let mut safe_lines = 0;
        for line in &input.lines {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_safe(&levels) {
                safe_lines += 1;
                continue;
            }

            for i in 0..levels.len() {
                let mut levels_removed = levels.clone();
                levels_removed.remove(i);

                if is_safe(&levels_removed) {
                    safe_lines += 1;
                    break;
                }
            }
        }
        Some(format!("{}", safe_lines))
    }
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let diffs = levels.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let steps_safe = diffs
        .iter()
        .map(|s| s.abs())
        .all(|diff| (1..=3).contains(&diff));
    let all_negative = diffs.iter().all(|diff| *diff < 0);
    let all_positive = diffs.iter().all(|diff| *diff > 0);
    steps_safe && (all_negative || all_positive)
}
