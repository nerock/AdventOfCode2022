use std::fs;

const BREAKPOINTS: [i32; 6] = [20, 60, 100, 140, 180, 220];
const NOOP: &str = "noop";
const ADD: &str = "addx";
const LIT: char = '#';
const NOT_LIT: char = '.';
const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    part_two(input.clone());
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

fn part_two(input: String) {
    let mut cycle = 1;
    let mut register = 1;
    let mut crt = vec![NOT_LIT; CRT_WIDTH*CRT_HEIGHT];

    for op in input.lines() {
        let (cycles, amount) = parse_op(op);
        for _ in 0..cycles {
            let crt_position = (cycle-1) % CRT_WIDTH;
            let should_draw = crt_position as i32 - register;

            if should_draw >= -1 && should_draw <= 1 {
                crt[cycle-1] = LIT;
            }

            cycle += 1;
        }

        register += amount;
    }

    for (i, pixel) in crt.iter().enumerate() {
        if i % CRT_WIDTH == 0 {
            print!("\n");
        }

        print!("{pixel}");
    }
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