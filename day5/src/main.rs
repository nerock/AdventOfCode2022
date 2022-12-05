use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}

fn part_one(input: String) -> String {
    let (stacks, instructions, number_stacks) = split_stacks_instructions(input);

    let mut stacks = get_stacks(stacks, number_stacks);
    stacks = move_stacks(stacks, instructions);

    let mut top = String::new();
    for stack in stacks {
        top.push(*stack.last().unwrap());
    }

    top
}

fn part_two(input: String) -> String {
    let (stacks, instructions, number_stacks) = split_stacks_instructions(input);

    let mut stacks = get_stacks(stacks, number_stacks);
    stacks = move_multiple_stacks(stacks, instructions);

    let mut top = String::new();
    for stack in stacks {
        top.push(*stack.last().unwrap());
    }

    top
}

fn split_stacks_instructions(input: String) -> (Vec<String>, Vec<String>, usize) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let number_stacks: Vec<&str> = input[0].split("\n").collect();
    let number_stacks = number_stacks[number_stacks.len()-1];
    let num = number_stacks.chars().nth(number_stacks.len()-1).unwrap();
    let last_num = num.to_digit(10).unwrap();

    let mut stacks: Vec<String> = input[0].split("\n").map(String::from).collect();
    stacks.pop();
    let instructions = input[1].split("\n").map(String::from).collect();

    (stacks, instructions, last_num as usize)
}

fn get_stacks(input: Vec<String>, n: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n];
    for line in input {
        let mut debug = String::new();
        let mut stack = 0;
        let mut spaces = 1;
        for elf_crate in line.chars() {
            debug.push(elf_crate);
            match elf_crate {
                ' ' => {
                    spaces += 1;
                    if spaces == 4 {
                        stack += 1;
                        spaces = 0;
                    }
                },
                '[' | ']' => {},
                _ => {
                    stacks[stack].push(elf_crate);
                    stack+=1;
                    spaces=0;
                },
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn move_stacks(stacks: Vec<Vec<char>>, instructions: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = stacks;
    for instruction in instructions {
        let (amount, from, to) = parse_instruction(instruction);

        for _ in 0..amount {
            let to_move = stacks[from-1].pop().unwrap();
            stacks[to-1].push(to_move);
        }
    }

    stacks
}

fn move_multiple_stacks(stacks: Vec<Vec<char>>, instructions: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = stacks;
    for instruction in instructions {
        let (amount, from, to) = parse_instruction(instruction);

        let mut to_move_stack = Vec::new();
        for _ in 0..amount {
            let to_move_crate = stacks[from-1].pop().unwrap();
            to_move_stack.push(to_move_crate);
        }

        to_move_stack.reverse();
        for to_move_crate in to_move_stack {
            stacks[to-1].push(to_move_crate);
        }
    }

    stacks
}

fn parse_instruction(instruction: String) -> (i32, usize, usize) {
    let instruction:Vec<&str> = instruction.split(" ").collect();
    let amount: i32 = instruction[1].parse().unwrap();
    let from: usize = instruction[3].parse().unwrap();
    let to: usize = instruction[5].parse().unwrap();

    (amount, from, to)
}