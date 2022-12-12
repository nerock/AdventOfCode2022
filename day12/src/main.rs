use std::collections::{HashMap, VecDeque};
use std::fs;

const START: char = 'S';
const BEST_SIGNAL: char = 'E';
const LOWEST: i8 = 'a' as i8;
const HIGHEST: i8 = 'z' as i8;

type Map = Vec<Vec<i8>>;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Heightmap {
    map: Map,
    starting_position: Point,
    best_signal: Point,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> u16 {
    let hmap = parse_heightmap(input);

    shortest_path(&hmap, hmap.starting_position.clone()).unwrap_or(0)
}

fn shortest_path(hmap: &Heightmap, current_position: Point) -> Option<u16> {
    let mut visited: HashMap<Point, u16> = HashMap::new();
    let mut to_visit: VecDeque<Point> = VecDeque::new();

    visited.insert(current_position, 0);
    to_visit.push_back(current_position);

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();
        for neighbour in get_neighbours(&hmap.map, &current) {
            if !visited.contains_key(&neighbour){
                to_visit.push_back(neighbour);
                visited.insert(neighbour, visited.get(&current).unwrap() + 1);

                if neighbour == hmap.best_signal {
                    return Some(visited.get(&current).unwrap() + 1);
                }
            }
        }
    }
    None
}

fn get_neighbours(hmap: &Map, pos: &Point) -> Vec<Point> {
    let mut neighbours = Vec::new();
    let height = hmap[pos.y][pos.x];

    if pos.x > 0 && hmap[pos.y][pos.x-1] - height <= 1 {
        neighbours.push(Point{x: pos.x-1, y: pos.y})
    }
    if pos.y > 0 && hmap[pos.y-1][pos.x] - height <= 1 {
        neighbours.push(Point{x: pos.x, y: pos.y-1})
    }
    if pos.x + 1 < hmap[pos.y].len() && hmap[pos.y][pos.x+1] - height <= 1 {
        neighbours.push(Point{x: pos.x+1, y: pos.y})
    }
    if pos.y + 1 < hmap.len() && hmap[pos.y+1][pos.x] - height <= 1 {
        neighbours.push(Point{x: pos.x, y: pos.y+1})
    }

    neighbours
}

fn parse_heightmap(input: &str) -> Heightmap {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let mut map: Map = vec![vec![0; width]; height];
    let mut current_position = Point{x: 0, y: 0};
    let mut best_signal = Point{x: 0, y: 0};

    for (i, line) in input.lines().enumerate() {
        for (j, height) in line.chars().enumerate() {
            match height {
                START => current_position = Point{x: j, y: i},
                BEST_SIGNAL => {
                    best_signal = Point{x:j, y: i};
                    map[i][j] = HIGHEST - LOWEST;
                },
                _ => map[i][j] = height as i8 - LOWEST,
            }
        }
    }

    Heightmap{
        map,
        starting_position: current_position,
        best_signal,
    }
}