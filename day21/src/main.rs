use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;

struct Monkey {
    op: char,
    n1: String,
    n2: String,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i64 {
    let mut monkey_numbers = HashMap::new();
    let mut waiting_monkeys = HashMap::new();
    for monkey in input.lines() {
        let (name, number) = monkey.split_once(":").unwrap();
        if is_op(number) {
            let name = String::from(name);
            let (op, n1, n2) = parse_operation(number);
            waiting_monkeys.insert(name, Monkey{op, n1, n2});
        } else {
            monkey_numbers.insert(name, parse_number(number));
        }
    }

    while !monkey_numbers.contains_key("root") {
        for (name, monkey) in &waiting_monkeys {
            let name = name.as_str();
            if monkey_numbers.contains_key(name) {
                continue
            }
            match get_monkey_number(monkey_numbers.get(&*monkey.n1), monkey_numbers.get(&*monkey.n2), monkey.op) {
                None => {}
                Some(n) => {
                    monkey_numbers.insert(&name, n);
                }
            }
        }
    }

    monkey_numbers["root"]
}

fn part_two(input: &str) -> i64 {
    let mut monkey_numbers = HashMap::new();
    let mut waiting_monkeys = HashMap::new();
    for monkey in input.lines() {
        let (name, number) = monkey.split_once(":").unwrap();
        if name == "humn" {
            continue
        }
        if is_op(number) {
            let name = String::from(name);
            let (op, n1, n2) = parse_operation(number);
            waiting_monkeys.insert(name, Monkey{op, n1, n2});
        } else {
            monkey_numbers.insert(name, parse_number(number));
        }
    }

    let root = waiting_monkeys.get("root").unwrap();
    while !monkey_numbers.contains_key(&*root.n1) && !monkey_numbers.contains_key(&*root.n2) {
        for (name, monkey) in &waiting_monkeys {
            let name = name.as_str();
            if monkey_numbers.contains_key(name) {
                continue
            }
            match get_monkey_number(monkey_numbers.get(&*monkey.n1), monkey_numbers.get(&*monkey.n2), monkey.op) {
                None => {}
                Some(n) => {
                    monkey_numbers.insert(&name, n);
                }
            }
        }
    }
    let mut name = if waiting_monkeys.contains_key(&*root.n1) { &*root.n1 } else { &*root.n2 };
    let mut result = if waiting_monkeys.contains_key(&*root.n1) { *monkey_numbers.get(&*root.n2).unwrap() } else { *monkey_numbers.get(&*root.n1).unwrap() };
    while name != "humn" {
        let monkey = waiting_monkeys.get(name).unwrap();
        let tmp_result = if monkey_numbers.contains_key(&*monkey.n1) { *monkey_numbers.get(&*monkey.n1).unwrap() } else { *monkey_numbers.get(&*monkey.n2).unwrap() };
        name = if !monkey_numbers.contains_key(&*monkey.n1) { &*monkey.n1 } else { &*monkey.n2 };
        if name == monkey.n2 && monkey.op == '-' {
            result = get_monkey_number(Some(&tmp_result), Some(&result), '-').unwrap();
        } else {
            result = get_monkey_number(Some(&result), Some(&tmp_result), opposite(monkey.op)).unwrap();
        }
    }

    result
}

fn get_monkey_number(n1: Option<&i64>, n2: Option<&i64>, op: char) -> Option<i64> {
    match (n1, n2) {
        (Some(n1), Some(n2)) => match op {
            '+' => Some(sum(*n1, *n2)),
            '*' => Some(mul(*n1, *n2)),
            '-' => Some(sub(*n1, *n2)),
            _ => Some(div(*n1, *n2)),
        },
        _ => None,
    }
}

fn is_op(op: &str) -> bool {
    op.contains("+") || op.contains("*") || op.contains("-") || op.contains("/")
}

fn parse_number(n: &str) -> i64 {
    n.trim().parse().unwrap()
}

fn parse_operation(operation: &str) -> (char, String, String) {
    let operation = operation.trim();
    let operation: Vec<&str> = operation.split_whitespace().collect();

    (operation[1].chars().next().unwrap(), String::from(operation[0]), String::from(operation[2]))
}

fn opposite(op: char) -> char {
    match op {
        '+' => '-',
        '-' => '+',
        '*' => '/',
        _ => '*',
    }
}

fn sum(i: i64, j: i64) -> i64 {
    i + j
}

fn sub(i: i64, j: i64) -> i64 {
    i - j
}

fn mul(i: i64, j: i64) -> i64 {
    i * j
}

fn div(i: i64, j: i64) -> i64 {
    i / j
}