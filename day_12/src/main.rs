use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32 
}

struct MapProblem {
    heights: Vec<Vec<i32>>,
    shortest_paths: Vec<Vec<i32>>,
    start_position: Point,
    end_position: Point
}

fn parse_input(filename: String) -> MapProblem {
    let file = File::open(filename).unwrap(); let reader = BufReader::new(file);
    let mut map = MapProblem {
        heights: vec![],
        shortest_paths: vec![],
        start_position: Point {x: 0, y: 0}, 
        end_position: Point {x: 0, y: 0}, 
    };
    let start_int = 'S' as i32;
    let end_int = 'E' as i32;
    let mut i = 0;
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap().chars().map(|c| c as i32).collect::<Vec<i32>>();
        let maybe_start_position = line.iter().position(|&x| x == start_int);
        let maybe_end_position = line.iter().position(|&x| x == end_int);
        if maybe_start_position.is_some() {
            map.start_position = Point {
                x: maybe_start_position.unwrap() as i32,
                y: i
            };
        }
        if maybe_end_position.is_some() {
            map.end_position = Point {
                x: maybe_end_position.unwrap() as i32,
                y: i
            }
        }
        map.shortest_paths.push(vec![0; line.len()]);
        map.heights.push(line);       
        i += 1; 
    }
    map.heights[map.start_position.y as usize][map.start_position.x as usize] = 1000;
    map.heights[map.end_position.y as usize][map.end_position.x as usize] = 'z' as i32 + 1;
    return map
}

fn get_new_heads(shortest_paths: &Vec<Vec<i32>>, heights: &Vec<Vec<i32>>, head: Point) -> Vec<Point> {
    let new_paths: Vec<Point> = vec![];
    let mut potential_new_paths = vec![
        Point {x: head.x + 1, y: head.y},
        Point {x: head.x - 1, y: head.y},
        Point {x: head.x, y: head.y + 1},
        Point {x: head.x, y: head.y - 1}
    ];
    println!("The potential_new_paths are {:?}", potential_new_paths.clone());
    let mut new_paths = vec![];
    
    let width = shortest_paths[0].len() as i32;
    let height = shortest_paths.len() as i32;
    println!("The head is {:?}", head.clone());
    println!("THe head x as usize is {:?}", head.x as usize);
    println!("THe head y as usize is {:?}", head.y as usize);
    let current_height = heights[head.y as usize][head.x as usize];
    for i in 0..potential_new_paths.len() {
        let potential_new_path = potential_new_paths.pop().unwrap();
        if (potential_new_path.x < 0) | (potential_new_path.x >= width) | (potential_new_path.y < 0) | (potential_new_path.y >= height)  {
            // Check for new head outside bounds
            continue;
        } else if (shortest_paths[potential_new_path.y as usize][potential_new_path.x as usize] > 0) {
            // Check for new head already searched
            continue;
        } else if heights[potential_new_path.y as usize][potential_new_path.x as usize].clone() > current_height + 1  {
            println!("The new potential height {} is greater than the current height at {}", heights[potential_new_path.y as usize][potential_new_path.x as usize].clone(), current_height);
            continue;
        } else {
            new_paths.push(potential_new_path);
        }
    }
    println!("Fetched {:?} new heads for point {:?}", new_paths.len(), head.clone());
    return new_paths
}


fn part_two(filename: String) {
}

fn part_one(filename: String) {
    let mut map_problem = parse_input(filename); 
    println!("The start position is {:?}", map_problem.start_position.clone());
    let mut current_path_heads = vec![map_problem.start_position];
    let mut path_length = 0;
    let mut new_heads = vec![];
    while true {
        path_length += 1;
        // Expand paths
        new_heads = vec![];
        for i in 0..current_path_heads.len() {
            let head = current_path_heads.pop().unwrap();
            new_heads.extend(get_new_heads(&map_problem.shortest_paths, &map_problem.heights, head));
            for head in new_heads.iter() {
                map_problem.shortest_paths[head.y as usize][head.x as usize] = path_length;
            }
            println!("The new heads are {:?}", new_heads.clone());
        }
        current_path_heads = new_heads;
        if current_path_heads.len() == 0 {
            break
        }
    }
    for line in map_problem.shortest_paths.iter() {
        println!("{:?}", line);
    }
    println!("The path to the end is length {:?}", map_problem.shortest_paths[map_problem.end_position.y as usize][map_problem.end_position.x as usize]);
    
}

fn main() {
    let filename: String = "input.txt".to_string();
    part_one(filename.clone());
    part_two(filename);
}
