use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    let (first, second, third) = part_two(input.clone());
    println!("{} {} {}", first, second, third);
}

fn part_one(input: String) -> i32 {
    let mut most_calories = 0;
    let mut current = 0;
    for calories in input.split("\n") {
        match calories {
            "" => {
                most_calories = if current > most_calories { current } else { most_calories };
                current = 0;
            }
            _ => current += get_calories(calories),
        }
    }

    most_calories
}

fn part_two(input: String) -> (i32, i32, i32) {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current = 0;

    for calories in input.split("\n") {
        match calories {
            "" => {
                (first, second, third) = get_top_three(current, first, second, third);
                current = 0;
            }
            _ => current += get_calories(calories),
        }
    }

    (first, second, third)
}

fn get_calories(calories: &str) -> i32 {
    calories.parse().expect("should be a number")
}

fn get_top_three(current: i32, first: i32, second: i32, third: i32) -> (i32, i32, i32) {
    let top_three: (i32, i32, i32);

    if current > first {
        top_three = (current, first, second);
    } else if current > second {
        top_three = (first, current, second);
    } else if current > third {
        top_three = (first, second, current);
    } else {
        top_three = (first, second, third)
    }

    top_three
}