use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").
        expect("Should have been able to read the input");

    let mut highest = 0;
    let mut current = 0;
    for line in input.split("\n") {
        match line {
            "" => {
                highest = if current > highest { current } else { highest };
                current = 0;
            },
            _ => current += line.parse::<i32>().unwrap(),
        }
    }

    println!("{highest}");
}

fn getTopThree(current, first, second, third: i32) -> (i32, i32, i32) {
    
}
