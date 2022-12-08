use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

   let tree_grid = parse_grid(input);
    println!("{}", part_one(&tree_grid));
    println!("{}", part_two(&tree_grid));
}

fn part_one(tree_grid: &Vec<Vec<u32>>) -> usize {
    let mut visible_trees = tree_grid.len() * tree_grid[0].len();
    for i in 1..tree_grid.len() {
        for j in 1..tree_grid[i].len() {
            if is_not_visible(&tree_grid, i, j) {
                visible_trees -= 1
            }
        }
    }

    visible_trees
}

fn part_two(tree_grid: &Vec<Vec<u32>>) -> i32 {
    let mut high_score = 0;

    for i in 0..tree_grid.len() {
        for j in 0..tree_grid[i].len() {
            let score = view_score(tree_grid, i, j);
            high_score = if score > high_score { score } else { high_score };
        }
    }

    high_score
}

fn parse_grid(input: String) -> Vec<Vec<u32>> {
    let mut tree_grid:Vec<Vec<u32>> = vec![];

    for tree_line in input.lines() {
        tree_grid.push(tree_line.chars().map(|t| t.to_digit(10).unwrap()).collect())
    }

    tree_grid
}

fn is_not_visible(tree_grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let height = tree_grid[i][j];

    let mut visible_top = true;
    let mut visible_bottom = true;
    let mut visible_left = true;
    let mut visible_right = true;

    for n in 0..tree_grid.len() {
        if n == i { // Check right and left
            for z in 0..tree_grid[n].len() {
                if z < j {
                    if height <= tree_grid[n][z] {
                        visible_left = false
                    }
                } else if z > j {
                    if height <= tree_grid[n][z] {
                        visible_right = false
                    }
                }
            }
        } else if n < i {
            if height <= tree_grid[n][j] {
                visible_top = false
            }
        } else {
            if height <= tree_grid[n][j] {
                visible_bottom = false
            }
        }
    }

    return !visible_left && !visible_bottom && !visible_right && !visible_top
}

fn view_score(tree_grid: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
    let height = tree_grid[i][j];

    let mut score_top = 0;
    let mut score_bottom = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    for n in 0..tree_grid.len() {
        if n == i { // Check right and left
            for z in 0..tree_grid[n].len() {
                if z < j {
                    if tree_grid[n][z] >= height {
                        score_left = 0;
                    }

                    score_left += 1;
                } else if z > j {
                    score_right += 1;
                    if tree_grid[n][z] >= height {
                        break
                    }
                }
            }
        } else if n < i {
            if tree_grid[n][j] >= height{
                score_top = 0;
            }

            score_top += 1;
        } else {
            score_bottom += 1;
            if tree_grid[n][j] >= height {
                break
            }
        }
    }

    score_right*score_top*score_bottom*score_left
}