use std::fs;

const AIR: char = '.';
const ROCK: char = '#';
const SAND: char = '+';
const SAND_START: (usize, usize) = (500, 0);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(&input));
}

fn part_one(input: &str) -> u16 {
    let (grid, resting) = fall(parse(input));

    resting
}


fn parse(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split(" -> ").collect();
        let (x, y): (&str, &str) = line[0].split_once(",").unwrap();
        let (mut x, mut y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());

        grid = add_points(grid, x, y, 0, 0);
        for pos in &line[1..] {
            let (nx, ny): (&str, &str) = pos.split_once(",").unwrap();
            let (nx, ny): (i32, i32) = (nx.parse().unwrap(), ny.parse().unwrap());

            grid = add_points(grid, x, y, nx-x, ny-y);
            (x, y) = (nx, ny)
        }
    }

    grid
}

fn add_points(mut grid: Vec<Vec<char>>, x: i32, y: i32, xdiff: i32, ydiff: i32) -> Vec<Vec<char>> {
    grid = extend_vec(grid, x+xdiff, y+ydiff);

    if xdiff != 0 {
        let start = if x > x+xdiff { x+xdiff } else { x };
        let end = if x < x+xdiff { x+xdiff } else { x };
        for i in start..=end {
            let y = y as usize;
            let i = if i >= 0 { i as usize } else { 0 };
            grid[y][i] = ROCK
        }
    } else {
        let start = if y > y+ydiff { y+ydiff } else { y };
        let end = if y < y+ydiff { y+ydiff } else { y };
        for i in start..=end {
            let x = x as usize;
            let i = if i >= 0 { i as usize } else { 0 };
            grid[i][x] = ROCK
        }
    }


    grid
}

fn fall(mut grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, u16) {
    let (mut x, mut y) = SAND_START;
    let mut resting = 0;

    loop {
        if y+1 == grid.len() {
            break
        }

        if grid[y+1][x] == AIR {
            y+=1
        } else if x == 0 {
            break
        } else if grid[y+1][x-1] == AIR {
            x-=1
        } else if grid[y+1][x+1] == AIR {
            x+=1
        } else {
            grid[y][x] = SAND;
            (x, y) = SAND_START;
            resting += 1
        }
    }

    (grid, resting)
}

// unnecessary to grow the vec so much and keep it squared BUT
fn extend_vec(mut grid: Vec<Vec<char>>, x: i32, y: i32) -> Vec<Vec<char>> {
    let y = if y >= 0 { y as usize } else { 0 };
    let x = if x >= 0 { x as usize } else { 0 };

    if y >= grid.len() {
        for _ in 0..(y-grid.len())+1 {
            grid.push(vec!['.'; x+1])
        }
    }

    for i in 0..grid.len() {
        if x >= grid[i].len() {
            for _ in 0..(x-grid[i].len())+1 {
                grid[i].push('.')
            }
        }
    }

    grid
}