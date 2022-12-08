use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct elfRange {
    first_elf_start: i32,
    first_elf_end: i32,
    second_elf_start: i32,
    second_elf_end: i32
}

impl elfRange {
    fn first_elf(&self) -> Vec<i32> {
        return vec![self.first_elf_start, self.first_elf_end]
    }
    fn second_elf(&self) -> Vec<i32> {
        return vec![self.second_elf_start, self.second_elf_end]
    }
}


fn parse_line(line: String) -> elfRange {

    let sector_ranges = line.split(',').collect::<Vec<&str>>();
    
    let first_sector_range = sector_ranges[0].to_string().clone();
    let second_sector_range = sector_ranges[1].to_string().clone();

    let second_elf_sectors = second_sector_range.split('-').collect::<Vec<&str>>();
    let first_elf_sectors = first_sector_range.split('-').collect::<Vec<&str>>();

    let first_elf_start = first_elf_sectors[0].parse::<i32>().unwrap();
    let first_elf_end = first_elf_sectors[1].parse::<i32>().unwrap();

    let second_elf_start = second_elf_sectors[0].parse::<i32>().unwrap();
    let second_elf_end = second_elf_sectors[1].parse::<i32>().unwrap();

    return elfRange { 
        first_elf_start,
        first_elf_end, 
        second_elf_start, 
        second_elf_end 
    }
}

fn check_complete_overlap(elf_range: elfRange) -> bool {
    if (elf_range.first_elf_start >= elf_range.second_elf_start && elf_range.first_elf_end <= elf_range.second_elf_end) | (elf_range.second_elf_start >= elf_range.first_elf_start && elf_range.second_elf_end <= elf_range.first_elf_end) {
        return true
    }
    return false
}

fn check_partial_overlap(elf_range: elfRange) -> bool {
    for val in elf_range.first_elf() {
        if (elf_range.second_elf_end >= val && val >= elf_range.second_elf_start) {
            return true
        } 
    }
    for val in elf_range.second_elf() {
        if (elf_range.first_elf_end >= val && val >= elf_range.first_elf_start) {
            return true
        } 
    }
    return false
}

fn part_one() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut no_overlapped = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        println!("The line is {}", line_str.clone());
        let elf_range = parse_line(line_str);
        if check_complete_overlap(elf_range) {
            println!("This line is overlapped");
            no_overlapped += 1;
        }
    }
    println!("The total is {}", no_overlapped);
}

fn part_two() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut no_overlapped = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        println!("The line is {}", line_str.clone());
        let elf_range = parse_line(line_str);
        if check_partial_overlap(elf_range) {
            println!("This line is overlapped");
            no_overlapped += 1;
        }
    }
    println!("The total is {}", no_overlapped);
}

fn main() {
    part_one();
    part_two();
}
