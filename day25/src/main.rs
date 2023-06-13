use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> String {
    to_snafu(input.lines().map(|snafu| to_decimal(snafu)).sum())
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |decimal, snafu_digit| {
        let decimal_digit = ['=', '-', '0', '1', '2']
            .into_iter()
            .position(|c| c == snafu_digit)
            .unwrap() as i64
            - 2;
        decimal * 5 + decimal_digit
    })
}

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return String::new();
    }

    let decimal_remainder = decimal % 5;
    let snafu_digit = ['0', '1', '2', '=', '-'][decimal_remainder as usize];

    let new_decimal = (decimal + 2) / 5;
    let mut snafu = to_snafu(new_decimal);
    snafu.push(snafu_digit);

    snafu
}