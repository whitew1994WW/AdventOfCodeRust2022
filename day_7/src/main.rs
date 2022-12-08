use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default)]
struct FileObject {
    files: Vec<FileObject>,
    size: i32,
    name: String
}

impl FileObject {
    fn new_file(size: i32, name: String) -> Self {
        return FileObject {
            size, 
            name, 
            files: vec![]
        }
    }

    fn new_folder(name: String) -> Self {
        return FileObject {
            size: 0,
            name, 
            files: vec![]
        }
    }
}

enum RowType {
    LS,
    CD,
    DIR,
    FILE
}


struct Row {
    row_type: RowType,
    name: String,
    size: i32,
}

impl Row {
    fn from_string(row_string: String) -> Self {
        let row_items = row_string.split(" ").collect::<Vec<&str>>();
        let row_type: RowType;
        let cmd_row_start = "$";
        let dir_row_start = "dir";
        match row_items[0] {
            cmd_row_start => {
                match row_items[1] {
                    "cd" => return Row {
                        row_type: RowType::CD,
                        name: row_items[2].to_string(),
                        size: 0
                    },
                    "ls" => return Row {
                        row_type: RowType::LS,
                        name: row_items[2].to_string(),
                        size: 0
                    }
                }
            }, 
            dir_row_start => return Row {
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

fn part_one(filepath: String) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let file_system = FileObject {
        files: vec![],
        name: "/".to_string(),
        size: 0
    };

    for line in reader.lines() {
        let row = Row::from_string(line.unwrap());
        
    }
}

fn main() {
    part_one();
    part_two();
}
