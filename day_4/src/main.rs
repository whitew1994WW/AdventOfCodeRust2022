use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn check_overlap(line: String) -> bool {
    let sector_ranges = line.split(',').collect::<Vec<&str>>();
    
    let first_sector_range = sector_ranges[0].to_string().clone();
    let second_sector_range = sector_ranges[1].to_string().clone();

    let second_elf_sectors = second_sector_range.split('-').collect::<Vec<&str>>();
    let first_elf_sectors = first_sector_range.split('-').collect::<Vec<&str>>();

    let first_range_start = first_elf_sectors[0].parse::<i32>().unwrap();
    let first_range_end = first_elf_sectors[1].parse::<i32>().unwrap();

    let second_range_start = second_elf_sectors[0].parse::<i32>().unwrap();
    let second_range_end = second_elf_sectors[1].parse::<i32>().unwrap();
    if (first_range_start >= second_range_start && first_range_end <= second_range_end) | (second_range_start >= first_range_start && second_range_end <= first_range_end) {
        println!("The first_range_start is {}, end {}, the second_range_start is {}, end {}", first_range_start, first_range_end, second_range_start, second_range_end);
        return true
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
        if check_overlap(line_str) {
            println!("This line is overlapped");
            no_overlapped += 1;
        }
    }
    println!("The total is {}", no_overlapped);
}

fn part_two() {

}

fn main() {
    part_one();
    part_two();
}
