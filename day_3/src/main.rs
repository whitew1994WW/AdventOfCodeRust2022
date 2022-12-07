use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn char_val_to_priority(char_val: i32) -> i32{
    if char_val < 49 {
        return char_val + 10
    } else {
        return char_val - 48
    }
}
fn part_one() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut score = 0;
    for line in reader.lines() {
        println!("The line is {:?}", line);
        let chars = line.unwrap().chars().map(|ch| ch as i32 - 0x30).map(|x| char_val_to_priority(x)).collect::<Vec<i32>>();
        println!("The chars are {:?}", chars);
        let midpoint = chars.len() / 2;
        for i in midpoint..chars.len() {
           if chars[..midpoint].contains(&chars[i]) {
                score = chars[i];
                println!("The char in both is {}", chars[i]);
                break;
           } 
        }
        total += score;
    }
    println!("The total is {}", total);
}

fn part_two() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut elf_group = vec![];
    for line in reader.lines() {
        println!("The line is {:?}", line);
        let chars = line.unwrap().chars().map(|ch| ch as i32 - 0x30).map(|x| char_val_to_priority(x)).collect::<Vec<i32>>();
        println!("The chars are {:?}", chars);
        elf_group.push(chars);
        if elf_group.len() == 3 {
            let mut first_two_common = vec![];
            for ch in elf_group[0].clone() {
                if elf_group[1].contains(&ch) {
                    first_two_common.push(ch);
                }
            }
            for ch in first_two_common {
                if elf_group[2].contains(&ch) {
                    println!("The common element in both is {}", ch);
                    total += ch;
                    break
                }
            }
            elf_group = vec![];
        }
        
    }
    println!("The total is {}", total);

}

fn main() {
    part_one();
    part_two();
}
