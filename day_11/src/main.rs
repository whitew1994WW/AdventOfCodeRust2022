use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<i128>,
    operation: Box<dyn Fn(i128) -> i128>,
    test_divisible: i128,
    true_monkey: usize,
    false_monkey: usize,
    monkey_no: usize
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
         .field("items", &self.items)
         .field("test_divisible", &self.test_divisible)
         .field("true_monkey", &self.true_monkey)
         .field("false_monkey", &self.false_monkey)
         .field("monkey_no", &self.monkey_no)
         .finish()
    }
}

impl Monkey {
    fn from_lines(lines: Vec<String>, monkey_no: usize) -> Self {
        let mut item_strings = lines[1].split(" ").collect::<Vec<&str>>(); 
        let mut items = VecDeque::new();
        while item_strings.len() > 2 {
            let to_parse = item_strings.pop().unwrap().trim_end_matches(",");
            println!("Parsing {} to int32", to_parse.clone());
            println!("The len of the item to parse is {}", to_parse.len());
            items.push_front(to_parse.parse::<i128>().unwrap());
        }
        let ops_strings = lines[2].split(" ").collect::<Vec<&str>>();
        let operation = parse_operation(ops_strings[3..6].to_owned());
        let mut test_divisible_strings = lines[3].split(" ").collect::<Vec<&str>>(); 
        let test_divisible = test_divisible_strings.pop().unwrap().parse::<i128>().unwrap();
        let mut true_monkey_strings = lines[4].split(" ").collect::<Vec<&str>>();
        let true_monkey = true_monkey_strings.pop().unwrap().parse::<usize>().unwrap();
        let mut false_monkey_strings = lines[5].split(" ").collect::<Vec<&str>>();
        let false_monkey = false_monkey_strings.pop().unwrap().parse::<usize>().unwrap();
        return Monkey { 
            items,
            operation, 
            test_divisible, 
            true_monkey, 
            false_monkey, 
            monkey_no 
        }
    }

    fn apply_operation(&self, input: i128) -> i128 {
        (self.operation)(input) 
    }
}

fn parse_operation(ops: Vec<&str>) -> Box<dyn Fn(i128) -> i128> {
    let second_arg_old = match ops[2] {
       "old" => true,
       _ => false
    };
    match ops[1] {
        "+" => {
            if second_arg_old {
                return Box::new(|x| x*2);
            } else {
                let second_arg = ops[2].parse::<i128>().unwrap();
                Box::new(move |x| x + second_arg)
            }
        },
        "*" => {
            if second_arg_old {
                return Box::new(|x| x*x);
            } else {
                let second_arg = ops[2].parse::<i128>().unwrap();
                Box::new(move |x| x * second_arg)
            }
        },
        _ => panic!("Did not expect op {}", ops[1])
    } 
}

fn parse_monkeys(filename: String) -> Vec<Monkey> {
    let file = File::open(filename).unwrap(); let reader = BufReader::new(file);
    let mut monkey_counter = 0;
    let mut monkey_no = 0;
    let mut monkey_lines: Vec<String> = vec![];
    let mut monkies: Vec<Monkey> = vec![];
    // Parse the monkies
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap().trim().to_string();
        monkey_lines.push(line);
        if monkey_counter > 5 {
            let monkey = Monkey::from_lines(monkey_lines.clone(), monkey_no.clone()); 
            monkies.push(monkey);
            monkey_counter = 0;        
            monkey_no += 1;
            monkey_lines = vec![];
        } else {
            monkey_counter += 1;
        } 
    }
    let monkey = Monkey::from_lines(monkey_lines.clone(), monkey_no.clone()); 
    monkies.push(monkey);
    
    println!("The monkey_lines list is {:?}", monkey_lines);
    return monkies
}

fn part_one() {
    let mut monkies = parse_monkeys("./input.txt".to_string());
    let mut monkey_inspections = vec![0; monkies.len()];
    for round in 0..20 {
        for monkey_no in 0..monkies.len() {
            while true {
                // Get item to pass
                let item_to_pass = {
                    let monkey = &mut monkies[monkey_no];
                    let maybe_item_to_inspect = monkey.items.pop_front();
                    let mut item_to_inspect = match maybe_item_to_inspect {
                        Some(item_to_inspect) => item_to_inspect,
                        None => break
                    };
                    monkey_inspections[monkey_no] += 1;
                    item_to_inspect = monkey.apply_operation(item_to_inspect); 
                    let bored_item = (item_to_inspect as f64) / 3.0;
                    let rounded_item = bored_item.floor() as i128;
                    rounded_item
                };
                // Check divisble
                let monkey_no_to_pass = {
                    let monkey = &mut monkies[monkey_no];
                    match item_to_pass % monkey.test_divisible {
                        0 => monkey.true_monkey,
                        _ => monkey.false_monkey
                    }
                };
                // Pass to new monkey
                {
                    let monkey_recipient = &mut monkies[monkey_no_to_pass];
                    monkey_recipient.items.push_back(item_to_pass)
                } 
            }
        }
    }
    println!("The monkies are: {:?}", monkies);
    println!("THe monkey inspections are {:?}", monkey_inspections);
}

fn part_two() {
    let mut monkies = parse_monkeys("./input.txt".to_string());
    let common_factor: i128 = monkies.iter().map(|x| x.test_divisible as u32).product::<u32>() as i128;
    let mut monkey_inspections = vec![0; monkies.len()];
    for round in 0..10000 {
        for monkey_no in 0..monkies.len() {
            while true {
                // Get item to pass
                let item_to_pass = {
                    let monkey = &mut monkies[monkey_no];
                    let maybe_item_to_inspect = monkey.items.pop_front();
                    let mut item_to_inspect = match maybe_item_to_inspect {
                        Some(item_to_inspect) => item_to_inspect,
                        None => break
                    };
                    monkey_inspections[monkey_no] += 1;
                    item_to_inspect = monkey.apply_operation(item_to_inspect); 
                    item_to_inspect
                };
                // Check divisble
                let monkey_no_to_pass = {
                    let monkey = &mut monkies[monkey_no];
                    match item_to_pass % monkey.test_divisible {
                        0 => monkey.true_monkey,
                        _ => monkey.false_monkey
                    }
                };
                // Pass to new monkey
                {
                    let monkey_recipient = &mut monkies[monkey_no_to_pass];
                    let new_item = item_to_pass % common_factor;
                    monkey_recipient.items.push_back(new_item);
                } 
            }
        }
    }
    println!("The monkies are: {:?}", monkies);
    println!("THe monkey inspections are {:?}", monkey_inspections);
}

fn main() {
    part_one();
    part_two();
}
