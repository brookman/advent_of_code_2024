use crate::common::*;

pub struct S;

use lazy_static::lazy_static;
use regex::Regex;
use z3::{
    self,
    ast::{Ast, Int},
    Config, Context, Optimize, SatResult,
};

lazy_static! {
    static ref BUTTON: Regex = Regex::new(r"^Button .: X(\+\d+), Y(\+\d+)$").unwrap();
    static ref PRIZE: Regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        solve(input, 0).to_string()
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
        solve(input, 10000000000000).to_string()
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

fn solve(input: &PuzzleInput, c: i64) -> i64 {
    let get_numbers = |regex: &Regex, string: &String| -> (i64, i64) {
        let cap = regex.captures_iter(string).next().unwrap();
        (
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        )
    };

    let mut result = 0;

    let cfg = Config::new();
    let ctx = &Context::new(&cfg);
    let a = Int::new_const(ctx, "a");
    let b = Int::new_const(ctx, "b");
    let zero = Int::from_i64(ctx, 0);
    let three = Int::from_i64(ctx, 3);
    let c = Int::from_i64(ctx, c);

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

        let (ax, ay) = get_numbers(&BUTTON, button_a);
        let (bx, by) = get_numbers(&BUTTON, button_b);
        let (px, py) = get_numbers(&PRIZE, prize);

        let ax = Int::from_i64(ctx, ax);
        let ay = Int::from_i64(ctx, ay);
        let bx = Int::from_i64(ctx, bx);
        let by = Int::from_i64(ctx, by);
        let px = Int::from_i64(ctx, px);
        let py = Int::from_i64(ctx, py);

        let opt = Optimize::new(ctx);

        opt.assert(&a.gt(&zero));
        opt.assert(&b.gt(&zero));
        opt.minimize(&Int::add(ctx, &[&Int::mul(ctx, &[&a, &three]), &b]));

        let eq_1 = Int::add(
            ctx,
            &[&Int::mul(ctx, &[&ax, &a]), &Int::mul(ctx, &[&bx, &b])],
        );
        let px = Int::add(ctx, &[&px, &c]);
        opt.assert(&eq_1._eq(&px));

        let eq_2 = Int::add(
            ctx,
            &[&Int::mul(ctx, &[&ay, &a]), &Int::mul(ctx, &[&by, &b])],
        );
        let py = Int::add(ctx, &[&py, &c]);
        opt.assert(&eq_2._eq(&py));

        if opt.check(&[]) == SatResult::Sat {
            let model = opt.get_model().unwrap();
            let a = model.eval(&a, true).unwrap().as_i64().unwrap();
            let b = model.eval(&b, true).unwrap().as_i64().unwrap();
            result += a * 3 + b;
        }
    }

    result
}
