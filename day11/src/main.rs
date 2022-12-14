use std::fs;

const ROUNDS_PART1: usize = 20;
const ROUNDS_PART2: usize = 10000;

type OP = fn(i64, i64) -> i64;

struct Test {
    test_value: i64,
    if_true: usize,
    if_false: usize,
}

struct Monkey {
    items: Vec<i64>,
    operation: (OP, Option<i64>),
    test: Test,
    inspections: i64,
}

impl Monkey {
    fn inspect(&mut self, old: i64) -> i64 {
        self.inspections += 1;

        let (op, n) = &self.operation;
        let n = match n {
            Some(n) => *n,
            None => old,
        };

        op(old, n)
    }

    fn test(&self, item: i64) -> usize {
        if item % self.test.test_value == 0 { self.test.if_true } else { self.test.if_false }
    }

    fn add_item(&mut self, item: i64) {
        self.items.push(item);
    }

    fn clear_items(&mut self) {
        self.items.clear();
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..ROUNDS_PART1 {
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

fn part_two(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);
    // only works because all divisors are prime numbers
    let least_common_multiple = monkeys.iter().map(|m| m.test.test_value).product::<i64>();

    for _ in 0..ROUNDS_PART2 {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let item = monkeys[m].items[i];
                let item = monkeys[m].inspect(item) % least_common_multiple;
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

fn parse_items(starting_items: &str) -> Vec<i64> {
    let starting_items: Vec<&str> = starting_items.split(": ").collect();
    let starting_items = starting_items[1];
    let mut worry_levels = vec![];

    for item in starting_items.split(", ") {
        worry_levels.push(item.parse().unwrap())
    }

    worry_levels
}

fn parse_operation(operation: &str) -> (OP, Option<i64>) {
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

fn top_two_active(monkeys: &Vec<Monkey>) -> (i64, i64) {
    let mut top_active: [i64; 2] = [0, 0];
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

fn sum(old: i64, amount: i64) -> i64 {
    old + amount
}

fn sub(old: i64, amount: i64) -> i64 {
    old - amount
}

fn mul(old: i64, amount: i64) -> i64 {
    old * amount
}

fn div(old: i64, amount: i64) -> i64 {
    old / amount
}

fn noop(old: i64, _: i64) -> i64 {
    old
}