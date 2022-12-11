use std::fs;

const ROUNDS: usize = 20;

type OP = fn(i32, i32) -> i32;

struct Test {
    test_value: i32,
    if_true: usize,
    if_false: usize,
}

struct Monkey {
    items: Vec<i32>,
    operation: (OP, Option<i32>),
    test: Test,
    inspections: i32,
}

impl Monkey {
    fn inspect(&mut self, old: i32) -> i32 {
        self.inspections += 1;

        let (op, n) = &self.operation;
        let n = match n {
            Some(n) => *n,
            None => old,
        };

        op(old, n)
    }

    fn test(&self, item: i32) -> usize {
        if item % self.test.test_value == 0 { self.test.if_true } else { self.test.if_false }
    }

    fn add_item(&mut self, item: i32) {
        self.items.push(item);
    }

    fn clear_items(&mut self) {
        self.items.clear();
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..ROUNDS {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let item = monkeys[m].items[i];
                let item = monkeys[m].inspect(item) / 3;
                let pass_to = monkeys[m].test(item);

                monkeys[pass_to].add_item(item);
            }

            monkeys[m].clear_items();
        }
    }

    let top_two_active = top_two_active(&monkeys);

    top_two_active.0*top_two_active.1
}

fn parse_monkeys(monkeys: &str) -> Vec<Monkey> {
    let mut monkey_vec = vec![];
    for monkey in monkeys.split("\n\n") {
        monkey_vec.push(parse_monkey(monkey));
    }

    monkey_vec
}

fn parse_monkey(monkey: &str) -> Monkey {
    let monkey: Vec<&str> = monkey.split("\n").collect();

    Monkey{
        items: parse_items(monkey[1]),
        operation: parse_operation(monkey[2]),
        test: parse_test(&monkey[3..]),
        inspections: 0,
    }
}

fn parse_items(starting_items: &str) -> Vec<i32> {
    let starting_items: Vec<&str> = starting_items.split(": ").collect();
    let starting_items = starting_items[1];
    let mut worry_levels = vec![];

    for item in starting_items.split(", ") {
        worry_levels.push(item.parse().unwrap())
    }

    worry_levels
}

fn parse_operation(operation: &str) -> (OP, Option<i32>) {
    let operation = operation.replace("Operation: new = old ", "");
    let operation = operation.trim();
    let (op, num) = operation.split_once(" ").unwrap();

    let num = match num {
        "old" => None,
        _ => Some(num.parse().unwrap()),
    };

    match op {
        "+" => (sum, num),
        "-" => (sub, num),
        "*" => (mul, num),
        "/" => (div, num),
        _ => (noop, None),
    }
}

fn parse_test(test: &[&str]) -> Test {
    Test{
        test_value: test[0].split(" ").last().unwrap().parse().unwrap(),
        if_true:test[1].split(" ").last().unwrap().parse().unwrap(),
        if_false: test[2].split(" ").last().unwrap().parse().unwrap(),
    }
}

fn top_two_active(monkeys: &Vec<Monkey>) -> (i32, i32) {
    let mut top_active: [i32; 2] = [0, 0];
    for monkey in monkeys {
        if monkey.inspections > top_active[0] {
            top_active[1] = top_active[0];
            top_active[0] = monkey.inspections;
        } else if monkey.inspections > top_active[1] {
            top_active[1] = monkey.inspections;
        }
    }

    (top_active[0], top_active[1])
}

fn sum(old: i32, amount: i32) -> i32 {
    old + amount
}

fn sub(old: i32, amount: i32) -> i32 {
    old - amount
}

fn mul(old: i32, amount: i32) -> i32 {
    old * amount
}

fn div(old: i32, amount: i32) -> i32 {
    old / amount
}

fn noop(old: i32, _: i32) -> i32 {
    old
}