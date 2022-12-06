use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}

fn part_one(input: String) -> usize {
    let mut last_four: [char; 4] = ['a'; 4];
    let stream: Vec<char> = input.chars().collect();
    
    for i in 0..stream.len() {
        last_four[i%4] = stream[i];
        if i >= 3 && check_valid_code(&last_four) {
           return i+1
        }
    }

    return 0
}

fn part_two(input: String) -> usize {
    let mut last_14: [char; 14] = ['a'; 14];
    let stream: Vec<char> = input.chars().collect();

    for i in 0..stream.len() {
        last_14[i%14] = stream[i];
        if i >= 3 && check_valid_code(&last_14) {
            return i+1
        }
    }

    return 0
}

fn check_valid_code(last_chars: &[char]) -> bool {
    for i in 0..last_chars.len()-1 {
        for j in i+1..last_chars.len() {
            if last_chars[i] == last_chars[j] {
                return false
            }
        }
    }

    return true
}