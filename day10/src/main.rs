use std::fs;

const BREAKPOINTS: [i32; 6] = [20, 60, 100, 140, 180, 220];
const NOOP: &str = "noop";
const ADD: &str = "addx";

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
}

fn part_one(input: String) -> i32 {
    let mut cycle = 1;
    let mut register = 1;

    let mut total = 0;
    for op in input.lines() {
        let (cycles, amount) = parse_op(op);
        for _ in 0..cycles {
            if BREAKPOINTS.contains(&cycle) {
                total += register*cycle;
            }

            cycle += 1;
        }

        register += amount;
    }

    total
}

fn parse_op(op: &str) -> (usize, i32) {
    if op.starts_with(NOOP) {
        (1, 0)
    } else if op.starts_with(ADD) {
        (2, op.split_once(" ").unwrap().1.parse().unwrap())
    } else {
        (0, 0)
    }
}