use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
}

fn part_one(input: String) -> u32 {
    let mut priorities = 0;

    for rucksack in input.split("\n") {
        let half = rucksack.len() / 2;
        let first_compartment = &rucksack[..half];
        let second_compartment = &rucksack[half..];

        let mut present_types = Vec::new(); 
        for item_type in first_compartment.chars() {
            if second_compartment.contains(item_type) && !present_types.contains(&item_type) {
                present_types.push(item_type);
                priorities += calculate_priority(item_type);
            }
        }
    }

    return priorities;
}

fn calculate_priority(item_type: char) -> u32 {
    if item_type < 'a' {
        return item_type as u32 - 'A' as u32 + 27;
    }

   return item_type as u32 - 'a' as u32 + 1;
}