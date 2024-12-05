use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

use crate::common::*;

pub struct S;

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let mut rules = vec![];
        let mut updates = vec![];

        let mut iter = input.lines.iter();
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split('|');
            let a = parts.next().unwrap().parse::<usize>().unwrap();
            let b = parts.next().unwrap().parse::<usize>().unwrap();
            rules.push((a, b));
        }

        for line in iter {
            let parts: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            updates.push(parts);
        }

        let mut must_be_before = HashMap::<usize, HashSet<usize>>::new();
        for (a, b) in &rules {
            must_be_before.entry(*a).or_default().insert(*b);
        }

        let mut valid_updates = vec![];
        let mut invalid_updates = vec![];

        let empty_set: HashSet<usize> = HashSet::new();

        for update in &updates {
            let mut valid = true;
            let mut alreay_seen = HashSet::new();
            for u in update {
                let must_be_before_u = must_be_before.get(u).unwrap_or(&empty_set);

                if alreay_seen.intersection(must_be_before_u).next().is_some() {
                    valid = false;
                }

                alreay_seen.insert(*u);
            }
            if valid {
                valid_updates.push(update);
            } else {
                invalid_updates.push(update);
            }
        }
        let mut result = 0;

        for valid_update in &valid_updates {
            if valid_update.len() % 2 == 0 {
                panic!("not even")
            }
            let middle_element = valid_update[valid_update.len() / 2];
            result += middle_element;
        }

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
    }

    fn expected_output_one(&self) -> &str {
        "143"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let mut rules = vec![];
        let mut updates = vec![];

        let mut iter = input.lines.iter();
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split('|');
            let a = parts.next().unwrap().parse::<usize>().unwrap();
            let b = parts.next().unwrap().parse::<usize>().unwrap();
            rules.push((a, b));
        }

        for line in iter {
            let parts: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            updates.push(parts);
        }

        let mut must_be_before = HashMap::<usize, HashSet<usize>>::new();
        for (a, b) in &rules {
            must_be_before.entry(*a).or_default().insert(*b);
        }

        let mut valid_updates = vec![];
        let mut invalid_updates = vec![];

        let empty_set: HashSet<usize> = HashSet::new();

        for update in &updates {
            let mut valid = true;
            let mut alreay_seen = HashSet::new();
            for u in update {
                let must_be_before_u = must_be_before.get(u).unwrap_or(&empty_set);

                if alreay_seen.intersection(must_be_before_u).next().is_some() {
                    valid = false;
                }

                alreay_seen.insert(*u);
            }
            if valid {
                valid_updates.push(update);
            } else {
                invalid_updates.push(update);
            }
        }

        let mut result = 0;

        for invalid_update in &invalid_updates {
            let sorted_invalid_update = invalid_update
                .iter()
                .sorted_by(|a, b| {
                    let must_be_before_a = must_be_before.get(a).unwrap_or(&empty_set);
                    if must_be_before_a.contains(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .collect::<Vec<_>>();

            if sorted_invalid_update.len() % 2 == 0 {
                panic!("not even")
            }
            let middle_element = sorted_invalid_update[sorted_invalid_update.len() / 2];
            result += middle_element;
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
    }

    fn expected_output_two(&self) -> &str {
        "123"
    }
}
