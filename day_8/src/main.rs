use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<i32>>,
    visible: Vec<Vec<i32>>
}

fn parse_forest(filepath: String) -> Forest {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut rows = vec![];
    let mut visible = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().map(|c| c as i32 - '0' as i32).collect::<Vec<i32>>();
        visible.push(vec![0; row.len()]);
        rows.push(row);
    }
    return Forest { rows, visible }
}

fn mark_visible(forest: &mut Forest) {
    let mut max_height_up = vec![-1; forest.rows[0].len()];
    let mut max_height_down = vec![-1; forest.rows[0].len()];
    let row_size = forest.rows.len();
    let column_size = forest.rows[0].len();
    for i in 0..row_size {
        let mut max_height_left = -1;
        let mut max_height_right = -1;
        for j in 0..column_size {
            // Left to right
            let tree_height = forest.rows[i][j];
            if tree_height > max_height_left {
                forest.visible[i][j] = 1;
                max_height_left = tree_height;
            }    
            // Top to bottom
            if tree_height > max_height_down[j] {
                   forest.visible[i][j] = 1;
               max_height_down[j] = tree_height;
            }
            // Right to left
            let tree_height = forest.rows[i][column_size - (j + 1)];
            if tree_height > max_height_right {
                forest.visible[i][column_size - (j + 1)] = 1;
                max_height_right = tree_height;
            }
            // Bottom to top
            let tree_height = forest.rows[row_size - (i + 1)][j];
            if tree_height > max_height_up[j] {
                forest.visible[row_size - (1 + i)][j] = 1;
                max_height_up[j] = tree_height;
            }
        }
    }
}

fn calc_scenery(x: usize, y: usize, forest: &Forest) -> i32 {
    let tree_height = forest.rows[y][x];
    // Look left
    let this_tree_height = tree_height.clone();
    let mut compare_x = x.clone();
    let mut other_tree_height = 0;
    let mut left_view_distance = 0;
    while compare_x > 0 && other_tree_height < this_tree_height {
        left_view_distance += 1;
        compare_x -= 1;
        other_tree_height = forest.rows[y][compare_x];
    } 
    
    // Look right 
    let this_tree_height = tree_height.clone();
    let mut compare_x = x.clone();
    let mut other_tree_height = 0;
    let mut right_view_distance = 0;
    while compare_x < forest.rows[0].len() - 1 && other_tree_height < this_tree_height {
        right_view_distance += 1;
        compare_x += 1;
        other_tree_height = forest.rows[y][compare_x];
    } 

    // Look up 
    let this_tree_height = tree_height.clone();
    let mut compare_y = y.clone();
    let mut other_tree_height = 0;
    let mut up_view_distance = 0;
    while compare_y > 0 && other_tree_height < this_tree_height {
        up_view_distance += 1;
        compare_y -= 1;
        other_tree_height = forest.rows[compare_y][x];
    } 

    // Look Down
    let this_tree_height = tree_height.clone();
    let mut compare_y = y.clone();
    let mut other_tree_height = 0;
    let mut down_view_distance = 0;
    while compare_y < forest.rows.len() - 1 && other_tree_height < this_tree_height {
        down_view_distance += 1;
        compare_y += 1;
        other_tree_height = forest.rows[compare_y][x];
    } 
    return left_view_distance * right_view_distance * up_view_distance * down_view_distance
}


fn count_visible(forest: &Forest) -> i32 {
    let mut total = 0;
    for vec in forest.visible.iter() {
        for val in vec.iter() {
            total += val;
        }
    }
    return total
}
fn print_visibility(forest: &Forest) {
    let row_size = forest.rows.len();
    println!("");
    for i in 0..row_size {
        println!("{:?}", forest.visible[i]);
    }
}
fn part_one() {
    let mut forest = parse_forest("./input.txt".to_string());
    mark_visible(&mut forest);
    let total_trees = count_visible(&forest);
    print_visibility(&forest);
    println!("The total trees are {}", total_trees);
}

fn part_two() {
    let mut forest = parse_forest("./input.txt".to_string());
    let mut highest_scenary_score = 0;
    let mut scenery_score = 0;
    for x in 0..forest.rows.len() {
        for y in 0..forest.rows[0].len() {
            scenery_score = calc_scenery(x, y, &forest);
            if scenery_score > highest_scenary_score {
                highest_scenary_score = scenery_score;
            }
        }
    }
    println!("The highest_scenary_score is {}", highest_scenary_score);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forest1() {
        let test_file = "./test_input.txt";


    }
}
