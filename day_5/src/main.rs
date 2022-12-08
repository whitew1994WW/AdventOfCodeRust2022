use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

struct Command {
    qty: i32,
    start: i32,
    end: i32
}

fn parse_line(line: String) -> Command {
    let words = line.split(' ').collect::<Vec<&str>>();
    let qty = words[1].parse::<i32>().unwrap();
    let start = words[3].parse::<i32>().unwrap();
    let end = words[5].parse::<i32>().unwrap();
    return Command {
        qty,
        start,
        end
    }
}


fn initialise_input() -> HashMap<i32, Vec<char>> {
    return HashMap::from([
        (1, vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N']),
        (2, vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G']),
        (3, vec!['L', 'Q', 'V']),
        (4, vec!['N', 'B', 'S', 'W', 'R', 'Q']),
        (5, vec!['N', 'D', 'F', 'T', 'V', 'M', 'B']),
        (6, vec!['M', 'D', 'B', 'V', 'H', 'T', 'R']),
        (7, vec!['D', 'B', 'Q', 'J']),
        (8, vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q']),
        (9, vec!['B', 'N', 'H', 'M', 'S'])
    ])
}

fn part_one() {

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut input = initialise_input();
    for line in reader.lines() {
       let cmd = parse_line(line.unwrap());
       for i in 0..cmd.qty {
           let to_move = input.get_mut(&cmd.start).unwrap().pop().unwrap();
           input.get_mut(&cmd.end).unwrap().push(to_move);
       }
    }
    println!("The final hashmap is {:?}", input)
}

fn part_two() {

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input = initialise_input();
    for line in reader.lines() {
        let cmd = parse_line(line.unwrap());
        let mut tmp_vec = vec![];
        for i in 0..cmd.qty {
            let to_move = input.get_mut(&cmd.start).unwrap().pop().unwrap();
            tmp_vec.push(to_move);
        }
        let mut stack = input.get_mut(&cmd.end).unwrap();
        while (!tmp_vec.is_empty()) {
            stack.push(tmp_vec.pop().unwrap());
        }
    }
    println!("The final stack is {:?}", input);
}

fn main() {
    part_one();
    part_two();
}
