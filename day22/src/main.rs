use std::collections::{HashMap, HashSet};
use std::fs;

const ELF: char = '#';

const ROUNDS: usize = 10;

const NORTH: (i32, i32) = (0, -1);
const EAST:  (i32, i32) = (1, 0);
const SOUTH: (i32, i32) = (0, 1);
const WEST:  (i32, i32) = (-1, 0);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, s) in line.chars().enumerate() {
            if s == ELF {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let directions = vec![NORTH, SOUTH, WEST, EAST];
    for i in 0..ROUNDS {
        let mut proposed_direction = HashMap::new();
        for elf in &elves {
            let mut available_direction = None;
            let mut free_pos = 0;
            for j in i..i+directions.len() {
                let dir = directions[j%directions.len()];
                if check_free(&elf, dir, &elves) {
                    if free_pos == 0 {
                        available_direction = Some(sum(&elf, dir));
                    }
                    free_pos += 1;
                }
            }
            available_direction = if free_pos < directions.len() { available_direction } else { None };
            proposed_direction.insert(*elf, available_direction.unwrap_or(*elf));
        }

        for (elf, new_pos) in &proposed_direction {
            let mut free = true;
            for (other_elf, pos) in &proposed_direction {
                if !equal(elf, other_elf) && equal(&new_pos, pos) {
                    free = false;
                    break;
                }
            }
            if free {
                elves.remove(elf);
                elves.insert(*new_pos);
            }
        }
    }

    let (x, y) = elves.iter().next().unwrap();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (*x, *x, *y, *y);
    for (x, y) in &elves {
        if *x > max_x {
            max_x = *x
        } else if *x < min_x {
            min_x = *x
        }

        if *y > max_y {
            max_y = *y
        } else if *y < min_y {
            min_y = *y
        }
    }

    (max_x+1 - min_x) * (max_y+1 - min_y) - elves.len() as i32
}

fn part_two(input: &str) -> i32 {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, s) in line.chars().enumerate() {
            if s == ELF {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let directions = vec![NORTH, SOUTH, WEST, EAST];
    let mut i = 0;
    loop {
        let mut elves_moved = false;
        let mut proposed_direction = HashMap::new();
        for elf in &elves {
            let mut available_direction = None;
            let mut free_pos = 0;
            for j in i..i+directions.len() {
                let dir = directions[j%directions.len()];
                if check_free(&elf, dir, &elves) {
                    if free_pos == 0 {
                        available_direction = Some(sum(&elf, dir));
                    }
                    free_pos += 1;
                }
            }
            if free_pos > 0 && free_pos < directions.len() {
                elves_moved = true;
            }
            available_direction = if free_pos < directions.len() { available_direction } else { None };
            proposed_direction.insert(*elf, available_direction.unwrap_or(*elf));
        }

        if !elves_moved {
            break;
        }
        for (elf, new_pos) in &proposed_direction {
            let mut free = true;
            for (other_elf, pos) in &proposed_direction {
                if !equal(elf, other_elf) && equal(&new_pos, pos) {
                    free = false;
                    break;
                }
            }
            if free {
                elves.remove(elf);
                elves.insert(*new_pos);
            }
        }

        i += 1;
    }

    i as i32 +1
}

fn check_free(pos: &(i32, i32), dir: (i32, i32), elves: &HashSet<(i32, i32)>) -> bool {
    match dir {
        NORTH | SOUTH => !elves.contains(&sum(pos, dir)) && !elves.contains(&sum(&sum(pos, dir), EAST)) && !elves.contains(&sum(&sum(pos, dir), WEST)),
        EAST | WEST => !elves.contains(&sum(pos,dir)) && !elves.contains(&sum(&sum(pos, dir), NORTH)) && !elves.contains(&sum(&sum(pos, dir), SOUTH)),
        _ => !elves.contains(&sum(pos,dir))
    }
}

fn sum(pos: &(i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (pos.0+dir.0, pos.1+dir.1)
}

fn equal(a: &(i32, i32), b: &(i32, i32)) -> bool {
    a.0 == b.0 && a.1 == b.1
}