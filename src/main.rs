use std::{fs, time::Instant};

use common::Solution;

mod common;
mod s_01;
mod s_02;
mod s_03;
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
    solve(s_25::S {}, "25");
    solve(s_24::S {}, "24");
    solve(s_23::S {}, "23");
    solve(s_22::S {}, "22");
    solve(s_21::S {}, "21");
    solve(s_20::S {}, "20");
    solve(s_19::S {}, "19");
    solve(s_18::S {}, "18");
    solve(s_17::S {}, "17");
    solve(s_16::S {}, "16");
    solve(s_15::S {}, "15");
    solve(s_14::S {}, "14");
    solve(s_13::S {}, "13");
    solve(s_12::S {}, "12");
    solve(s_11::S {}, "11");
    solve(s_10::S {}, "10");
    solve(s_09::S {}, "09");
    solve(s_08::S {}, "08");
    solve(s_07::S {}, "07");
    solve(s_06::S {}, "06");
    solve(s_05::S {}, "05");
    solve(s_04::S {}, "04");
    solve(s_03::S {}, "03");
    solve(s_02::S {}, "02");
    solve(s_01::S {}, "01");
}

fn solve<T: Solution>(solution: T, day: &str) {
    let input = fs::read_to_string(format!("input/{}.txt", day)).unwrap();
    if input.is_empty() {
        return;
    }
    let lines: Vec<&str> = input.lines().collect();

    println!("\nDecember {}, 2024", day);

    println!("--- Part One ---");
    let start = Instant::now();
    let result = solution.solve_one(&input, &lines);
    let elapsed = start.elapsed();
    if result.is_some() {
        println!("{}     in {:?}", result.unwrap(), elapsed);
    } else {
        println!("Not solved yet");
    }

    println!("--- Part Two ---");
    let start = Instant::now();
    let result = solution.solve_two(&input, &lines);
    let elapsed = start.elapsed();
    if result.is_some() {
        println!("{}     in {:?}", result.unwrap(), elapsed);
    } else {
        println!("Not solved yet");
    }
}
