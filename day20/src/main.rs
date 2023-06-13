use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> u32 {
    let mut numbers = get_numbers(input);
    let len = numbers.len() as i32;
    let mut indexes: Vec<usize> = vec![0; numbers.len()];
    for i in 0..indexes.len() {
        indexes[i] = i;
    }
    for i in 0..numbers.len() {
        let i = indexes[i];
        let n = numbers[i];
        indexes.swap(i, (i+n as usize)%numbers.len());
        if n > 0 {
            let n = n as usize;
            for j in i..i+n {
                let j = j % numbers.len();
                let next = (j+1) % numbers.len();
                indexes.swap(j, next);
                numbers.swap(j, next)
            }
        } else if n < 0 {
            let start = i as i32;
            let end = len + ((n+start) % len);
            if start > end {
                for j in (end..=start).rev() {
                    let j = if j < 0 { (len + j as i32) as usize } else { j as usize };
                    let previous = if j as i32 -1 < 0 { (len + j as i32 - 1) as usize } else { (j as i32 - 1) as usize };
                    indexes.swap(j,  previous);
                    numbers.swap(j, previous)
                }
            } else {
                for j in start..end-1 {
                    let j = j as usize;
                    indexes.swap(j,  j+1);
                    numbers.swap(j, j+1)
                }
            }
        }
    }

    println!("{:?}", numbers);
    0
}

fn get_numbers(input: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for n in input.lines() {

        numbers.push(n.parse().unwrap())
    }

    numbers
}