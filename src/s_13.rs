use crate::common::*;

pub struct S;

use lazy_static::lazy_static;
use regex::Regex;
use z3::{
    ast::{self, Ast},
    Config, Context, Optimize, SatResult,
};

lazy_static! {
    static ref BUTTON: Regex = Regex::new(r"^Button .: X(\+\d+), Y(\+\d+)$").unwrap();
    static ref PRIZE: Regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let mut result = 0;

        let mut iter = input.lines.iter();
        let mut line = iter.next();

        while line.is_some() {
            let button_a = line.unwrap();
            line = iter.next();
            let button_b = line.unwrap();
            line = iter.next();
            let prize = line.unwrap();
            iter.next();
            line = iter.next();

            let mut ax = 0;
            let mut ay = 0;
            let mut bx = 0;
            let mut by = 0;
            let mut px = 0;
            let mut py = 0;

            for mul in BUTTON.captures_iter(button_a) {
                ax = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                ay = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            }

            for mul in BUTTON.captures_iter(button_b) {
                bx = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                by = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            }

            for mul in PRIZE.captures_iter(prize) {
                px = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                py = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            }

            let mut cheapest = i32::MAX;

            for a in 1..=100 {
                for b in 1..=100 {
                    let x = ax * a + bx * b;
                    let y = ay * a + by * b;
                    if x == px && y == py {
                        let tokens = a * 3 + b;
                        cheapest = cheapest.min(tokens);
                    }
                }
            }

            if cheapest != i32::MAX {
                result += cheapest;
            }
        }

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#
    }

    fn expected_output_one(&self) -> &str {
        "480"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let mut result = 0;

        let mut iter = input.lines.iter();
        let mut line = iter.next();

        while line.is_some() {
            let button_a = line.unwrap();
            line = iter.next();
            let button_b = line.unwrap();
            line = iter.next();
            let prize = line.unwrap();
            iter.next();
            line = iter.next();

            let mut ax = 0;
            let mut ay = 0;
            let mut bx = 0;
            let mut by = 0;
            let mut px = 0;
            let mut py = 0;

            for mul in BUTTON.captures_iter(button_a) {
                ax = mul.get(1).unwrap().as_str().parse::<i64>().unwrap();
                ay = mul.get(2).unwrap().as_str().parse::<i64>().unwrap();
            }

            for mul in BUTTON.captures_iter(button_b) {
                bx = mul.get(1).unwrap().as_str().parse::<i64>().unwrap();
                by = mul.get(2).unwrap().as_str().parse::<i64>().unwrap();
            }

            for mul in PRIZE.captures_iter(prize) {
                px = mul.get(1).unwrap().as_str().parse::<i64>().unwrap();
                py = mul.get(2).unwrap().as_str().parse::<i64>().unwrap();
            }

            let cfg = Config::new();
            let ctx = Context::new(&cfg);

            let a = ast::Int::new_const(&ctx, "a");
            let b = ast::Int::new_const(&ctx, "b");
            let ax = ast::Int::from_i64(&ctx, ax);
            let ay = ast::Int::from_i64(&ctx, ay);
            let bx = ast::Int::from_i64(&ctx, bx);
            let by = ast::Int::from_i64(&ctx, by);
            let px = ast::Int::from_i64(&ctx, px);
            let py = ast::Int::from_i64(&ctx, py);

            let c = ast::Int::from_i64(&ctx, 10000000000000);
            let zero = ast::Int::from_i64(&ctx, 0);
            let three = ast::Int::from_i64(&ctx, 3);

            let solver = Optimize::new(&ctx);
            solver.assert(&a.gt(&zero));
            solver.assert(&b.gt(&zero));

            let eq_1 = ast::Int::add(
                &ctx,
                &[
                    &ast::Int::mul(&ctx, &[&ax, &a]),
                    &ast::Int::mul(&ctx, &[&bx, &b]),
                ],
            );
            let px = ast::Int::add(&ctx, &[&px, &c]);
            solver.assert(&eq_1._eq(&px));

            let eq_2 = ast::Int::add(
                &ctx,
                &[
                    &ast::Int::mul(&ctx, &[&ay, &a]),
                    &ast::Int::mul(&ctx, &[&by, &b]),
                ],
            );
            let py = ast::Int::add(&ctx, &[&py, &c]);
            solver.assert(&eq_2._eq(&py));

            let to_minimize = ast::Int::add(&ctx, &[&ast::Int::mul(&ctx, &[&a, &three]), &b]);
            solver.minimize(&to_minimize);

            if solver.check(&[]) == SatResult::Sat {
                let model = solver.get_model().unwrap();
                let a = model.eval(&a, true).unwrap().as_i64().unwrap();
                let b = model.eval(&b, true).unwrap().as_i64().unwrap();
                result += a * 3 + b;
            }
        }

        result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#
    }

    fn expected_output_two(&self) -> &str {
        "875318608908"
    }
}
