use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").
        expect("Should have been able to read the input");

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current = 0;
    for line in input.split("\n") {
        match line {
            "" => {
                (first, second, third) = get_top_three(current, first, second, third);
                current = 0;
            },
            _ => current += line.parse::<i32>().unwrap(),
        }
    }

    println!("{first}");
    println!("{}", first+second+third);
}

fn get_top_three(current: i32, first: i32, second: i32, third: i32) -> (i32, i32, i32) {
   if current > first {
       return (current, first, second)
   } else if current > second {
       return (first, current, second)
   } else if current > third {
       return (first, second, current)
   }

   return (first, second, third)
}
