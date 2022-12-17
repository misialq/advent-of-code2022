use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub fn solve(fp: &str) -> i32 {
    let result = read_file(fp);
    process_list(result)
}

struct ElfFile {
    name: String,
    size: i32,
}

struct ElfDirectory {
    name: String,
    level: i32,
    subdirs: Vec<Self>,
    files: Vec<ElfFile>,
    size: i32,
}

impl fmt::Debug for ElfDirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ElfDirectory '{}': size = {}", self.name, self.size)
    }
}

fn read_file(fp: &str) -> Vec<String> {
    println!("Reading in data from {}", fp);
    let file = File::open(fp).unwrap();
    let reader = BufReader::new(file);

    let mut contents = vec![];
    for line in reader.lines() {
        let l = line.unwrap();
        contents.push(l);
    }
    contents
}

fn process_list(lines: Vec<String>) -> i32 {
    let mut directories = vec![];
    directories.push(ElfDirectory {
        name: String::from("/"),
        size: 0,
        level: 0,
        subdirs: vec![],
        files: vec![],
    });

    let mut current_level = 0;
    let mut dir_stack: Vec<String> = vec!["/".to_string()];
    for line in lines.iter() {
        println!("{:?}", line);
        let mut current_dir = "/".to_owned();
        let mut previous_dir = "/".to_owned();
        if line.starts_with("$ cd") {
            println!("Init level: {}", current_level);
            if line.contains("/") {
                println!("Hello");
                //  jump to root
                // current_dir = &directories
                //     .iter()
                //     .filter(|x| x.name == "/")
                //     .collect::<Vec<ElfDirectory>>()[0]
                //     .name;
                previous_dir = current_dir.clone();
                current_dir = "/".to_owned();
                current_level = 0;
            } else if line.contains("..") {
                //  go up
                let x = previous_dir.clone();
                previous_dir = current_dir.clone();
                current_dir = x.clone();
                current_level = current_level - 1;
            } else {
                //  change the directory
                previous_dir = current_dir.clone();
                current_dir = line[5..].to_owned();
                current_level = current_level + 1;
            }
            println!("\tChanging dir to {:?} ({}), prev: {}", &current_dir, current_level, &previous_dir)
        }

        if line.starts_with("dir") {
            let dir_name = line[4..].to_owned();
            if !directories
                .iter()
                .map(|x| x.name.to_owned())
                .collect::<Vec<String>>()
                .contains(&dir_name)
            {
                directories.push(ElfDirectory {
                    name: dir_name,
                    size: 0,
                    level: current_level + 1,
                    subdirs: vec![],
                    files: vec![],
                })
            }
        }
    }
    println!("{:?}", directories);
    0
}
