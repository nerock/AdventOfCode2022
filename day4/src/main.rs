use std::fs;

struct Range {
    start: i32,
    end: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}

fn part_one(input: String) -> u32 {
    let mut fully_contained = 0;

    for pair in input.split("\n") {
        let (first, second) = get_pairs(pair);

        if (first.start <= second.start && first.end >= second.end)
            || (second.start <= first.start && second.end >= first.end)
        {
            fully_contained += 1;
        }
    }

    return fully_contained;
}

fn part_two(input: String) -> u32 {
    let mut overlap = 0;

    for pair in input.split("\n") {
        let (first, second) = get_pairs(pair);

        if first.start == second.start
            || first.start == second.end
            || first.end == second.start
            || first.end == second.end
        {
            overlap += 1;
        } else if first.start < second.start && first.end >= second.start {
            overlap += 1;
        } else if first.start > second.start && first.start <= second.end {
            overlap += 1;
        }
    }

    return overlap;
}

fn get_pairs(pair: &str) -> (Range, Range) {
    let pair: Vec<&str> = pair.split(",").collect();

    let (start, end) = get_range_start_end(pair[0]);
    let first = Range { start, end };

    let (start, end) = get_range_start_end(pair[1]);
    let second = Range { start, end };

    return (first, second);
}

fn get_range_start_end(range: &str) -> (i32, i32) {
    let range: Vec<&str> = range.split("-").collect();
    let start = range[0].parse().expect("Should be a valid number");
    let end = range[1].parse().expect("Should be a valid number");

    return (start, end);
}
