use std::collections::{HashMap, HashSet};
use std::fs;

struct XYPos {
    lowest_z: i32,
    highest_z: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
    //println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut cubes = HashSet::new();
    let mut surface = 0;

    for cube in input.lines() {
        let cube: Vec<&str> = cube.split(",").collect();
        let (x, y, z) = (cube[0].parse().unwrap(), cube[1].parse().unwrap(), cube[2].parse().unwrap());

        cubes.insert((x, y, z));
        surface += 6;

        for adjacent in gen_adjacents(x, y, z) {
            if cubes.contains(&adjacent) {
                surface -= 2
            }
        }
    }
    println!("{}", cubes.len());

    surface
}

fn part_two(input: &str) -> u32 {
    let mut cubes = HashSet::new();
    let mut surface_cubes = HashMap::new();
    let mut surface = 0;

    for cube in input.lines() {
        let cube: Vec<&str> = cube.split(",").collect();
        let (x, y, z) = (cube[0].parse().unwrap(), cube[1].parse().unwrap(), cube[2].parse().unwrap());

        cubes.insert((x, y, z));

        let mut n_cube = XYPos{highest_z: z, lowest_z: z};
        if surface_cubes.contains_key(&(x, y)) {
            let cube: &XYPos = surface_cubes.get(&(x, y)).unwrap();
            n_cube.highest_z = if z > cube.highest_z { z } else { cube.highest_z };
            n_cube.lowest_z = if z < cube.lowest_z { z } else { cube.lowest_z };
        }
        surface_cubes.insert((x, y), n_cube);
    }

    for ((x, y), cube) in surface_cubes {
        surface += 12;

        for (x, y, z) in [(x, y, cube.highest_z), (x, y, cube.lowest_z)] {
            for adjacent in gen_adjacents(x, y, z) {
                if cubes.contains(&adjacent) {
                    surface -= 2
                }
            }
        }
    }
    println!("{}", cubes.len());

    surface
}

fn gen_adjacents(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x+1, y, z),
        (x-1, y, z),
        (x, y+1, z),
        (x, y-1, z),
        (x, y, z+1),
        (x, y, z-1)
    ]
}