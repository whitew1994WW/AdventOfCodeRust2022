use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_one(){
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut total = 0;
    let mut current_max = 0;
    for line in reader.lines() {
        let maybe_line = line.unwrap();
        println!("{:?}", maybe_line);
        let maybe_int = maybe_line.parse::<i32>();
        if maybe_line.is_empty() {
            if total > current_max {
                println!("Skipping current line, new max is {:?}, greater than the old max {}", total.clone(), current_max);
                current_max = total.clone();
            }
            total = 0;
            continue;
        } else if let Ok(new_int) = maybe_int {
            println!("The parsed number is {:?}", new_int);
            total += new_int; 
        } else {
            panic!("Badly formatted line!")
        }
    }
    println!("The max is {}", current_max);
}

fn part_two() {

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut best_elves = vec![];
    let mut total = 0;
    for line in reader.lines() {
        let maybe_line = line.unwrap();
        println!("{:?}", maybe_line);
        let maybe_int = maybe_line.parse::<i32>();
        if maybe_line.is_empty() {
            if best_elves.len() == 0 {
                best_elves.insert(0, total);
                total = 0;
                continue;
            }

            for idx in 0..best_elves.len() {
                let elf_calories = best_elves[idx];
                if total > elf_calories {
                    best_elves.insert(idx, total);
                    if best_elves.len() > 3 {
                        best_elves = best_elves[..3].to_vec();
                    }
                    break;
                }
            }
            total = 0;
            continue;
        } else if let Ok(new_int) = maybe_int {
            println!("The parsed number is {:?}", new_int);
            total += new_int; 
        } else {
            panic!("Badly formatted line!")
        }
    }
    println!("The best_elves are {:?}", best_elves);
    let total_calories: i32 = best_elves.iter().sum();
    println!("The total the elves are carrying is {:?}", total_calories);
}

fn main() -> io::Result<()> {
    part_one();
    part_two();
    Ok(())
}
