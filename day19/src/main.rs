use std::fs;
use regex::Regex;

struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    let mut quality_level = 0;

    for (id, blueprint) in input.lines().enumerate() {
        quality_level = (id as i32 +1) * max_quality(blueprint);
    }

    quality_level
}

fn max_quality(blueprint: &str) -> i32 {
    let costs: Vec<&str> = blueprint.split(".").collect();
    let n_regex = Regex::new(r"[0-9]+").unwrap();

    let ore_robot_cost: i32 = n_regex.captures(costs[0].split_once(":").unwrap().1).unwrap()[0].parse().unwrap();
    let clay_robot_cost: i32 = n_regex.captures(costs[1]).unwrap()[0].parse().unwrap();
    let obisidan_robot_ore_cost: i32 = n_regex.captures(costs[2].split_once("ore").unwrap().0).unwrap()[0].parse().unwrap();
    let obisidan_robot_clay_cost: i32 = n_regex.captures(costs[2].split_once("ore").unwrap().1).unwrap()[0].parse().unwrap();
    let geode_robot_ore_cost: i32 = n_regex.captures(costs[3].split_once("ore").unwrap().0).unwrap()[0].parse().unwrap();
    let geodce_robot_clay_cost: i32 = n_regex.captures(costs[3].split_once("ore").unwrap().1).unwrap()[0].parse().unwrap();
    println!("{obisidan_robot_clay_cost}");



    ore_robot_cost
}