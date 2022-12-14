use std::fs;

const DRAW_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;

enum MatchResult {
    WIN,
    DRAW,
    LOSE,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}

fn part_one(input: String) -> i32 {
    let mut points: i32 = 0;
    for line in input.split("\n") {
        points += get_points(line.split_whitespace().collect());
    }

    points
}

fn part_two(input: String) -> i32 {
    let mut points: i32 = 0;
    for line in input.split("\n") {
        let moves: Vec<&str> = line.split_whitespace().collect();
        let elf_move = moves[0];
        let correct_move = get_move(moves);
        points += get_points(vec![elf_move, correct_move]);
    }

    points
}

fn get_points(moves: Vec<&str>) -> i32 {
    let points:i32 = match moves[1] {
        "X" => 1,
        "Y" => 2,
        _ => 3,
    };

    match calculate_winner(moves[0], moves[1]) {
        MatchResult::WIN => points + WIN_POINTS,
        MatchResult::DRAW => points + DRAW_POINTS,
        MatchResult::LOSE => points,
    }
}

fn get_move(input: Vec<&str>) -> &str {
    match input[0] {
        "A" => match input[1] {
            "X" => "Z",
            "Y" => "X",
            _ => "Y",
        },
        "C" => match input[1] {
            "X" => "Y",
            "Y" => "Z",
            _ => "X",
        },
        _ => input[1],
    }
}

fn calculate_winner(elf_move: &str, player_move: &str) -> MatchResult {
    match elf_move {
        "A" => match player_move {
            "X" => MatchResult::DRAW,
            "Y" => MatchResult::WIN,
            _ => MatchResult::LOSE,
        },
        "B" => match player_move {
            "X" => MatchResult::LOSE,
            "Y" => MatchResult::DRAW,
            _ => MatchResult::WIN
        }
        _ => match player_move {
            "X" => MatchResult::WIN,
            "Y" => MatchResult::LOSE,
            _ => MatchResult::DRAW,
        },
    }
}