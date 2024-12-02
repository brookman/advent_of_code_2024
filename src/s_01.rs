use std::collections::HashMap;

use crate::common::*;

pub struct S {}

impl Solution for S {
    fn test_one(&self) -> (&str, &str) {
        (
            r#"3   4
4   3
2   5
1   3
3   9
3   3
"#,
            "11",
        )
    }

    fn test_two(&self) -> (&str, &str) {
        (
            r#"3   4
4   3
2   5
1   3
3   9
3   3
"#,
            "31",
        )
    }

    fn solve_one(&self, input: &PuzzleInput) -> Option<String> {
        let (mut list_one, mut list_two) = parse_columns(input);

        list_one.sort();
        list_two.sort();

        let distance_sum: u32 = list_one
            .iter()
            .zip(list_two)
            .map(|(one, two)| (one - two).unsigned_abs())
            .sum();

        Some(format!("{}", distance_sum))
    }

    fn solve_two(&self, input: &PuzzleInput) -> Option<String> {
        let (list_one, list_two) = parse_columns(input);

        let frequency_map = list_two.iter().fold(HashMap::new(), |mut map, item| {
            *map.entry(*item).or_insert(0) += 1;
            map
        });

        let mut total_similarity = 0;
        for one in list_one {
            let occurences = frequency_map.get(&one).unwrap_or(&0);
            let similarity = one * occurences;
            total_similarity += similarity;
        }

        Some(format!("{}", total_similarity))
    }
}

fn parse_columns(input: &PuzzleInput) -> (Vec<i32>, Vec<i32>) {
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for line in &input.lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        list_one.push(parts[0].parse::<i32>().unwrap());
        list_two.push(parts[1].parse::<i32>().unwrap());
    }
    (list_one, list_two)
}
