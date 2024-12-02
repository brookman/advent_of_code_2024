use crate::common::*;

pub struct S {}

impl Solution for S {
    fn test_one(&self) -> (&str, &str) {
        (
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#,
            "2",
        )
    }

    fn test_two(&self) -> (&str, &str) {
        (
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#,
            "4",
        )
    }

    fn solve_one(&self, input: &PuzzleInput) -> Option<String> {
        let lines = input.parsed2d::<i32>();
        let safe_lines = lines.iter().filter(|line| is_safe(line)).count();
        Some(format!("{}", safe_lines))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let lines = input.parsed2d::<i32>();
        let safe_lines = lines.iter().filter(|line| is_safe_dampened(line)).count();
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
