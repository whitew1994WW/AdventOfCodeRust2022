use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Point {
    fn add_int(self, other: i32) -> Self {
        Self {
            x: self.x + other.clone(),
            y: self.y + other.clone()
        }
    }

    fn multiply_by_int(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

fn difference_to_tail_move(diff: Point) -> Point {
    if (diff.x.abs() == 1 && diff.y.abs() == 1) | (diff.x.abs() == 1 && diff.y == 0) | (diff.y.abs() == 1 && diff.x == 0) {
        // The touching case 
        return Point {x: 0, y: 0} 
    } else if  (diff.y == 0 && diff.x.abs() > 1)  {
        // The direct move for x 
        return Point { x: diff.x - diff.x.signum(), y: 0 } 
    } else if (diff.x == 0 && diff.y.abs() > 1) {
        // THe direct move for y
        return Point { x: 0, y: diff.y - diff.y.signum() } 
    } else if (diff.x == 0 && diff.y == 0) {
        // The no move
        return Point { x: 0, y: 0 }
    } else if (diff.x.abs() == 1 && diff.y.abs() == 2) | (diff.y.abs() == 1 && diff.x.abs() == 2) | (diff.x.abs() == 2 && diff.y.abs() == 2)  {
        // The knight chess move
        return Point {
            x: diff.x.signum(),
            y: diff.y.signum()
        }
    } else {
        println!("THe move is {:?}", diff);
        println!("X abs value is {}, y abs value is {}", diff.x.abs(), diff.y.abs());
        panic!("I should now have been asked to make this move");
    }
}

fn part_one() {
    let file = File::open("./input.txt").unwrap(); let reader = BufReader::new(file);
    let mut head_pos = Point { x: 0, y: 0 };
    let mut tail_pos = Point { x: 0, y: 0 };
    let mut tail_move: Point; 
    let mut head_tail_diff: Point;
    let mut tail_locations: HashSet<Point> = HashSet::new();
    tail_locations.insert(tail_pos.clone());
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        println!("The line is {:?}", line);
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let direction = match line_parts[0] {
            "D" => Point { x: 0, y: -1 },
            "U" => Point { x: 0, y: 1 },
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            _ => panic!("Incorrect input!")
        };
        let number_moves = line_parts[1].parse::<i32>().unwrap();
        println!("Moving {:?} moves in direction {:?}", number_moves, direction);
        for i in 0..number_moves {
            head_pos = head_pos.clone() + direction.clone();
            head_tail_diff = head_pos.clone() - tail_pos.clone();
            println!("The head_tail_diff is {:?}", head_tail_diff.clone());
            tail_move = difference_to_tail_move(head_tail_diff);
            tail_pos = tail_pos + tail_move;
            tail_locations.insert(tail_pos.clone());
            println!("The new head_pos is {:?}. THe new tail pos is {:?}.", head_pos.clone(), tail_pos.clone());
        }
    }
    println!("The number of tail locations are {:?}", tail_locations.len());
}

fn part_two() {
    let file = File::open("./input.txt").unwrap(); let reader = BufReader::new(file);
    let mut positions = vec![Point { x: 0, y: 0 }; 10];
    let mut tail_move: Point; 
    let mut head_tail_diff: Point;
    let mut tail_locations: HashSet<Point> = HashSet::new();
    tail_locations.insert(Point { x: 0, y: 0 });
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        println!("The line is {:?}", line);
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let direction = match line_parts[0] {
            "D" => Point { x: 0, y: -1 },
            "U" => Point { x: 0, y: 1 },
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            _ => panic!("Incorrect input!")
        };
        let number_moves = line_parts[1].parse::<i32>().unwrap();
        println!("Moving {:?} moves in direction {:?}", number_moves, direction);
        for i in 0..number_moves {
            // Move head
            positions[0] = positions[0].clone() + direction.clone();
            for head_idx in 0..positions.len()-1 {
                head_tail_diff = positions[head_idx].clone() - positions[head_idx + 1].clone();
                println!("The head_tail_diff for head {} and tail {} is {:?}", head_idx, head_idx + 1, head_tail_diff.clone());
                println!("The tail previous position is {:?}", positions[head_idx + 1].clone());
                tail_move = difference_to_tail_move(head_tail_diff);
                positions[head_idx + 1] = positions[head_idx + 1].clone() + tail_move;
                println!("The tail new position is {:?}", positions[head_idx + 1].clone());
            }
            tail_locations.insert(positions[9].clone());
            println!("THe new tail pos is {:?}.", positions[9].clone());
        }
    }
    println!("The number of tail locations are {:?}", tail_locations.len());
}

fn main() {
    part_one();
    part_two();
}
