use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_one(input.clone()));
}

fn part_one(input: String) -> usize {
    let mut tree_grid:Vec<Vec<u32>> = vec![];
    
    for tree_line in input.lines() {
        tree_grid.push(tree_line.chars().map(|t| t.to_digit(10).unwrap()).collect())
    }

    let mut visible_trees = tree_grid.len() * tree_grid[0].len();
    for i in 1..tree_grid.len() {
        for j in 1..tree_grid[i].len() {
            if is_not_visible(&tree_grid, i, j) {
                visible_trees -= 1
            }
        }
    }

    return visible_trees
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