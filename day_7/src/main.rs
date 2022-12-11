use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process::Child;

#[derive(Default, Clone, Debug)]
struct FileObject {
    // Maps the name of the file/folder to the location of the object in the flattened array of
    // files
    files: HashMap<String, usize>,
    size: i32,
    name: String,
    folder: usize,
    depth: i32
}

impl FileObject {
    fn new_file(size: i32, name: String, folder_idx: usize, depth: i32) -> Self {
        return FileObject {
            size, 
            name, 
            files: HashMap::new(),
            folder: folder_idx,
            depth
        }
    }

    fn new_folder(name: String, folder_idx: usize, depth: i32) -> Self {
        return FileObject {
            size: 0,
            name, 
            files: HashMap::new(),
            folder: folder_idx,
            depth
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
        name: "root".to_string(),
        size: 0,
        folder: 0,
        depth: 0
    };
    let mut file_directory = vec![root];
    let mut current_idx = 0;
    let mut current_max_idx = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let row = Row::from_string(line.unwrap());
        
        println!("The row is {:?}", row);    
        match row.row_type {
            RowType::CD => {
                current_idx = match row.name.as_str() {
                    ".." => {
                        depth -= 1;
                        file_directory[current_idx].folder
                    }
                    _ => {
                        depth += 1;
                        let mut current_dir = &mut file_directory[current_idx.clone()];
                        let maybe_child_idx = current_dir.files.get(&row.name);
                        match maybe_child_idx {
                           Some(child_idx) => {
                                child_idx.clone()
                           },
                           None => {
                                current_max_idx += 1;
                                current_dir.files.insert(row.name.clone(), current_max_idx.clone());
                                file_directory.push(FileObject { 
                                    files: HashMap::new(), 
                                    size: 0, 
                                    name: row.name, 
                                    folder: current_idx.clone(),
                                    depth: depth.clone()
                                });
                                current_max_idx.clone()
                           }
                        }
                    } 
                }; 
            },
            RowType::LS => {},
            RowType::DIR => {
                let current_dir = &mut file_directory[current_idx];
                let maybe_this_folder = current_dir.files.get(&row.name);
                match maybe_this_folder {
                    Some(this_folder) => {},
                    None => {
                        current_max_idx += 1;
                        current_dir.files.insert(row.name.clone(), current_max_idx.clone());
                        file_directory.push(FileObject { 
                            files: HashMap::new(), 
                            size: 0, 
                            name: row.name, 
                            folder: current_idx.clone(),
                            depth: depth.clone()
                        });
                    }
                }
            }
            RowType::FILE => {
                let current_dir = &mut file_directory[current_idx];
                let maybe_this_file = current_dir.files.get(&row.name);
                match maybe_this_file {
                    Some(this_folder) => {},
                    None => {
                        current_max_idx += 1;
                        current_dir.files.insert(row.name.clone(), current_max_idx.clone());
                        file_directory.push(FileObject { 
                            files: HashMap::new(), 
                            size: row.size, 
                            name: row.name, 
                            folder: current_idx.clone(),
                            depth: depth.clone()
                        });
                    }
                }
            }
        }
    }
    println!("The file directory is {:?}", file_directory);
    let mut total_size = 0;
    for file_idx in 0..file_directory.len() {
        let mut this_size = 0;
        let this_file = &file_directory[file_idx];
        if this_file.size > 0 {
            this_size = this_file.size; 
        } else {
            this_size = calculate_directory_size(&file_directory, file_idx); 
            let this_file = &mut file_directory[file_idx];
            this_file.size = this_size;
        } 
        if this_size <= 100000 {
            total_size += this_size;
        }
    }
    println!("The total size of all directories with a size less than 100000 is {:?}", total_size);
}

//fn find_files_greater_than(file_directory: &mut Vec<FileObject>, min_file_size: i64, ) -> Vec<FileObject> {
//    
//}

fn calculate_directory_size(file_directory: &Vec<FileObject>, current_file_idx: usize) -> i32 {
    let mut un_scanned_folders: Vec<usize> = vec![current_file_idx];
    let mut files: Vec<usize> = vec![];
    while un_scanned_folders.len() > 0 {
        let current_idx = un_scanned_folders.pop().unwrap();
        let current_folder = file_directory[current_idx].clone();
        let child_file_idxs = current_folder.files.into_values().collect::<Vec<usize>>();
        for idx in child_file_idxs.iter() {
            let file = file_directory[idx.clone()].clone();
            if file.size == 0 {
                un_scanned_folders.push(idx.clone());
            } else {
                files.push(idx.clone());
            } 
    }
    }

    // Sum up the files inside this folder
    let mut total_size = 0;
    for file_idx in files.iter() {
        let file = &file_directory[file_idx.clone()];
        total_size += file.size;
    } 
    return total_size
}

fn print_file_structure(file_directory: &Vec<FileObject>) {
    let mut un_printed_folders: Vec<usize> = vec![0];
    while un_printed_folders.len() > 0  {
        let current_idx = un_printed_folders.pop().unwrap();
        let current_file = &file_directory[current_idx];
        println!("{}{}  {}", "  ".repeat(depth.clone()), current_file.size, current_file.name); 
        for file_idx in current_file.files.into_values().collect::<Vec<usize>>() {
            child_file = file_directory[file_idx];
        }

    }
}

fn part_two() {}

fn main() {
    part_one("./input.txt".to_string());
    part_two();
}
