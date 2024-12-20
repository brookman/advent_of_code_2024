use crate::common::*;

pub struct S;

const TEST: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let lines = input.parsed2d::<i32>();
        let safe_lines = lines.iter().filter(|line| is_safe(line)).count();
        safe_lines.to_string()
    }

    fn test_input_one(&self) -> &str {
        TEST
    }

    fn expected_output_one(&self) -> &str {
        "2"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let lines = input.parsed2d::<i32>();
        let safe_lines = lines.iter().filter(|line| is_safe_dampened(line)).count();
        safe_lines.to_string()
    }

    fn test_input_two(&self) -> &str {
        TEST
    }

    fn expected_output_two(&self) -> &str {
        "4"
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

fn is_safe_dampened(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut levels_removed = Vec::from(levels);
        levels_removed.remove(i);

        if is_safe(&levels_removed) {
            return true;
        }
    }
    false
}
