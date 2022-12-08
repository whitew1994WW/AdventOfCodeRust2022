use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;


fn part_one() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut packet = String::new();
    for line in reader.lines() {
        packet = line.unwrap();
    }
    for i in 0..packet.chars().count() {
        let string_slice = packet[i..i+4].to_string();
        let string_set = string_slice.chars().collect::<HashSet<char>>();
        if string_set.len() == 4 as usize {
            println!("The start of the opening packet is at {}", i);
            return
        }
    }
}

fn part_two() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut packet = String::new();
    for line in reader.lines() {
        packet = line.unwrap();
    }
    for i in 0..packet.chars().count() {
        let string_slice = packet[i..i+14].to_string();
        let string_set = string_slice.chars().collect::<HashSet<char>>();
        if string_set.len() == 14 as usize {
            println!("The start of the opening packet is at {}", i);
            return
        }
    }
}

fn main() {
    part_one();
    part_two();
}
