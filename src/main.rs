use std::{
    thread,
    time::{Duration, Instant},
    vec,
};

use common::{PuzzleInput, Solution};
use itertools::Itertools;

mod common;
mod s_01;
mod s_02;
mod s_03;
mod s_03_2;
mod s_04;
mod s_05;
mod s_06;
mod s_07;
mod s_08;
mod s_09;
mod s_10;
mod s_11;
mod s_12;
mod s_13;
mod s_14;
mod s_15;
mod s_16;
mod s_17;
mod s_18;
mod s_19;
mod s_20;
mod s_21;
mod s_22;
mod s_23;
mod s_24;
mod s_25;

fn main() {
    let solutions: Vec<Vec<Box<dyn Solution>>> = vec![
        vec![Box::new(s_01::S)],
        vec![Box::new(s_02::S)],
        vec![Box::new(s_03::S), Box::new(s_03_2::S)],
        vec![Box::new(s_04::S)],
        vec![Box::new(s_05::S)],
        vec![Box::new(s_06::S)],
        vec![Box::new(s_07::S)],
        vec![Box::new(s_08::S)],
        vec![Box::new(s_09::S)],
        vec![Box::new(s_10::S)],
        vec![Box::new(s_11::S)],
        vec![Box::new(s_12::S)],
        vec![Box::new(s_13::S)],
        vec![Box::new(s_14::S)],
        vec![Box::new(s_15::S)],
        vec![Box::new(s_16::S)],
        vec![Box::new(s_17::S)],
        vec![Box::new(s_18::S)],
        vec![Box::new(s_19::S)],
        vec![Box::new(s_20::S)],
        vec![Box::new(s_21::S)],
        vec![Box::new(s_22::S)],
        vec![Box::new(s_23::S)],
        vec![Box::new(s_24::S)],
        vec![Box::new(s_25::S)],
    ];

    // newest first
    let solutions_per_day = solutions
        .into_iter()
        .enumerate()
        .map(|(i, solution)| (i + 1, solution))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .rev()
        .collect::<Vec<_>>();

    let mut wait = true;
    for (i, solutions) in solutions_per_day {
        let result = solve(solutions, format!("{:02}", i).as_str());
        if result.is_some() && wait {
            // wait a few seconds after printing the latest solution
            thread::sleep(Duration::from_secs(5));
            wait = false;
        }
    }
}

fn solve(solutions: Vec<Box<dyn Solution>>, day: &str) -> Option<()> {
    let input = PuzzleInput::new(&format!("input/{}.txt", day));
    input.as_ref()?;
    let input = input.unwrap();
    if input.input.is_empty() {
        return None;
    }

    println!("\nDecember {}, 2024", day);

    let has_multple_solutions = solutions.len() > 1;

    for (i, solution) in solutions.iter().enumerate() {
        if has_multple_solutions {
            println!("--- Solution {}, Part One ---", i + 1);
        } else {
            println!("--- Part One ---");
        }

        let test_input = solution.test_input_one();
        let expected_output = solution.expected_output_one();

        if !test_input.is_empty() {
            let test_input = PuzzleInput::from_str(test_input).unwrap();
            let actual_output = solution.solve_one(&test_input);
            if !actual_output.is_empty() {
                assert_eq!(actual_output, expected_output, "test for part one failed");
            }
        }
        let start = Instant::now();
        let result = solution.solve_one(&input);
        let elapsed = start.elapsed();
        if !result.is_empty() {
            println!("{}     in {:?}", result, elapsed);
        } else {
            println!("Not solved yet");
        }

        if has_multple_solutions {
            println!("--- Solution {}, Part Two ---", i + 1);
        } else {
            println!("--- Part Two ---");
        }

        let test_input = solution.test_input_two();
        let expected_output = solution.expected_output_two();

        if !test_input.is_empty() {
            let test_input = PuzzleInput::from_str(test_input).unwrap();
            let actual_output = solution.solve_two(&test_input);
            if !actual_output.is_empty() {
                assert_eq!(actual_output, expected_output, "test for part two failed");
            }
        }
        let start = Instant::now();
        let result = solution.solve_two(&input);
        let elapsed = start.elapsed();
        if !result.is_empty() {
            println!("{}     in {:?}", result, elapsed);
        } else {
            println!("Not solved yet");
        }
    }
    Some(())
}
