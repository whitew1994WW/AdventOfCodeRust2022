use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default)]
struct FileObject {
    // Maps the name of the file/folder to the location of the object in the flattened array of
    // files
    files: HashMap<String, usize>,
    size: i32,
    name: String,
    folder: usize 
}

impl FileObject {
    fn new_file(size: i32, name: String, folder_idx: usize) -> Self {
        return FileObject {
            size, 
            name, 
            files: HashMap::new(),
            folder: folder_idx
        }
    }

    fn new_folder(name: String, folder_idx: usize) -> Self {
        return FileObject {
            size: 0,
            name, 
            files: HashMap::new(),
            folder: folder_idx
        }
    }
}

#[derive(Debug)]
enum RowType {
    LS,
    CD,
    DIR,
    FILE
}

#[derive(Debug)]
struct Row {
    row_type: RowType,
    name: String,
    size: i32,
}

impl Row {
    fn from_string(row_string: String) -> Self {
        let row_items = row_string.split(" ").collect::<Vec<&str>>();
        let row_type: RowType;
        match row_items[0] {
            "$" => {
                match row_items[1] {
                    "cd" => return Row {
                        row_type: RowType::CD,
                        name: row_items[2].to_string(),
                        size: 0
                    },
                    "ls" => return Row {
                        row_type: RowType::LS,
                        name: "".to_string(),
                        size: 0
                    },
                    _ => panic!("This is an unexpected command, cmd_items: {:?}", row_items)
                }
            }, 
            "dir" => return Row {
                row_type: RowType::DIR,
                name: row_items[1].to_string(),
                size: 0
            },
            _ => return Row {
                row_type: RowType::FILE,
                name: row_items[1].to_string(),
                size: row_items[0].parse::<i32>().unwrap()
            }
        }      
    }
}

fn part_one(filepath: String) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let root = FileObject {
        files: HashMap::new(),
        name: "/".to_string(),
        size: 0,
        folder: 0
    };
    let mut file_directory = vec![root];
    let mut current_idx = 0;
    let mut current_max_idx = 0;
    for line in reader.lines() {
        let row = Row::from_string(line.unwrap());
        
        println!("The row is {:?}", row);    
        match row.row_type {
            RowType::CD => {
                current_idx = match row.name.as_str() {
                    ".." => file_directory[current_idx].folder,
                    _ => &mut file_directory[current_folder.files.get(&row.name).unwrap().clone()] 
                }; 
            },
            RowType::LS => {

            },
            RowType::DIR => {
            
            },
            RowType::FILE => {

            }
        }
    
    }
}

fn part_two() {}

fn main() {
    part_one("./input.txt".to_string());
    part_two();
}
