use std::fs;

const DRAW: i32 = 3;
const WIN: i32 = 6;

fn main() {
    let input = fs::read_to_string("input.txt").
    expect("Should have been able to read the input");
    
    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let mut points: i32 = 0;
    for line in input.split("\n") {
        points += get_points(line.split_whitespace().collect());
    }

    println!("{points}");
}

fn part_two(input: String) {
    let mut points: i32 = 0;
    for line in input.split("\n") {
        let moves: Vec<&str> = line.split_whitespace().collect();
        let elf_move = moves[0];
        let correct_move = get_move(moves);
        points += get_points(vec![elf_move, correct_move]);
    }

    println!("{points}");
}

fn get_points(moves: Vec<&str>) -> i32 {
    let points: i32 = if moves[1] == "X" { 1 } else if moves[1] == "Y" { 2 } else { 3 };
    if (moves[0] == "A" && moves[1] == "Y") || (moves[0] == "B" && moves[1] == "Z") || (moves[0] == "C" && moves[1] == "X") {
        return points + WIN;
    } else if (moves[0] == "A" && moves[1] == "X") || (moves[0] == "B" && moves[1] == "Y") || (moves[0] == "C" && moves[1] == "Z") {
        return points + DRAW;
    }

    return points;
}

fn get_move(input: Vec<&str>) -> &str {
    if input[0] == "A" {
        if input[1] == "X" {
            return "Z";
        } else if input[1] == "Y" {
            return "X";
        } else {
            return "Y";
        }
    } else if input[0] == "B" {
       return input[1];
    } else {
        if input[1] == "X" {
            return "Y";
        } else if input[1] == "Y" {
            return "Z";
        } else {
            return "X";
        }
    }
}