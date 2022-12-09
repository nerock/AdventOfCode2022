use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
}

fn part_one(input: String) -> usize {
    let mut head = Pos{ x: 0, y: 0 };
    let mut tail = Pos{ x: 0, y: 0 };

    let mut visited_locations = HashSet::new();
    visited_locations.insert((tail.x, tail.y));

    let mut past_head = Pos{x: head.x, y: head.y};
    for move_line in input.lines() {
        let (dir, amount) = parse_move(move_line);
        for _ in 0..amount {
            match dir {
                Direction::UP => head.y += 1,
                Direction::DOWN => head.y -= 1,
                Direction::LEFT => head.x -= 1,
                Direction::RIGHT => head.x += 1,
            }

            if !is_touching(&head, &tail) {
                tail = past_head;
            }
            past_head = head;

            visited_locations.insert((tail.x, tail.y));
        }
    }

    visited_locations.len()
}

fn parse_move(move_line: &str) -> (Direction, i32) {
    let move_line: Vec<&str> = move_line.split_whitespace().collect();

    (match move_line[0] {
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        "L" => Direction::LEFT,
        _ => Direction::RIGHT,
    }, move_line[1].parse().unwrap())
}

fn is_touching(head: &Pos, tail: &Pos) -> bool {
    let distance_x = if head.x - tail.x < 0 { (head.x - tail.x) * -1 } else { head.x - tail.x };
    let distance_y = if head.y - tail.y < 0 { (head.y - tail.y) * -1 } else { head.y - tail.y };

    if distance_x <= 1 && distance_y <= 1 { true } else { false }
}