use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn check_cycle_stop(cycle_stops: &Vec<i32>, cycle_number: &i32, signal_strengths: &mut Vec<i32>, X_register: &i32) {
    if cycle_stops.contains(cycle_number) {
        let signal_strength = cycle_number * X_register;
        signal_strengths.push(signal_strength);
    }
}


fn part_one() {
    let file = File::open("./input.txt").unwrap(); let reader = BufReader::new(file);
    let mut cycle_number = 1;
    let mut X_register = 1;
    let cycle_stops: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strengths: Vec<i32> = vec![];
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        match line_parts[0] {
            "noop" => {
                println!("Noop!");
                cycle_number += 1;
                check_cycle_stop(&cycle_stops, &cycle_number, &mut signal_strengths, &X_register);
                println!("Cycle Number <= {}", cycle_number.clone());
            },
            "addx" => {
                println!("Addx => {}", line_parts[1].clone());
                cycle_number += 1;
                println!("Cycle Number <= {}", cycle_number.clone());
                check_cycle_stop(&cycle_stops, &cycle_number, &mut signal_strengths, &X_register);
                X_register += line_parts[1].parse::<i32>().unwrap();
                cycle_number += 1;
                check_cycle_stop(&cycle_stops, &cycle_number, &mut signal_strengths, &X_register);
                println!("Cycle Number <= {}", cycle_number.clone());
                println!("X <= {}", X_register.clone());
            },
            _ => panic!("invalid command {}", line) 
        }
    }
    let total_signal_strength: i32 = signal_strengths.iter().sum();
    println!("The total signal_strength is {}", total_signal_strength);
}

#[derive(Debug, Clone)]
struct Index {
    row_no: i32,
    col_no: i32
}

fn get_crt_char(index: &Index, X_register: &i32) -> char {
    if index.col_no >= X_register -1 && index.col_no <= X_register +1 {
        return '#'
    }
    return '.'
}

fn get_next_index(index: Index) -> Index {
    let mut new_index = index.clone();
    if index.col_no == 39 {
        new_index.col_no = 0;
        if index.row_no == 5 {
            new_index.row_no = 0;
        } else {
            new_index.row_no = index.row_no + 1;
        }
    } else {
        new_index.col_no = index.col_no + 1;
    }
    println!("The new index is {:?}", index);
    return new_index
}

fn part_two() {
    let file = File::open("./input.txt").unwrap(); let reader = BufReader::new(file);
    let mut cycle_number = 0;
    let mut X_register = 1;
    let mut crt_grid: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut index = Index { row_no: 0, col_no: 0 };
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        match line_parts[0] {
            "noop" => {
                println!("Noop!");
                cycle_number += 1;
                crt_grid[index.row_no as usize][index.col_no as usize] = get_crt_char(&index, &X_register);
                index = get_next_index(index);
                println!("Cycle Number <= {}", cycle_number.clone());
            },
            "addx" => {
                cycle_number += 1;
                println!("Addx => {}", line_parts[1].clone());
                crt_grid[index.row_no as usize][index.col_no as usize] = get_crt_char(&index, &X_register);
                index = get_next_index(index);
                println!("Cycle Number <= {}", cycle_number.clone());
                cycle_number += 1;
                crt_grid[index.row_no as usize][index.col_no as usize] = get_crt_char(&index, &X_register);
                index = get_next_index(index);
                X_register += line_parts[1].parse::<i32>().unwrap();
                println!("Cycle Number <= {}", cycle_number.clone());
                println!("X <= {}", X_register.clone());
            },
            _ => panic!("invalid command {}", line) 
        }
    }
    for i in 0..6 {
        println!("{:?}", crt_grid[i].iter().collect::<String>());
    } 
}

fn main() {
    part_one();
    part_two();
}
