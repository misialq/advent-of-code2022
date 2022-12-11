use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub fn solve(fp: &str) -> usize {
    let result = read_file(fp);
    process_list(result)
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

fn is_marker(token: &str) -> bool {
    let mut unique: Vec<char> = vec![];
    for t in token.chars().into_iter() {
        if !unique.contains(&t) {
            unique.push(t)
        }
    }
    if token.len() == unique.len() {
        true
    } else {
        false
    }
}

fn process_list(lines: Vec<String>) -> usize {
    let line = &lines[0];
    let window_size = 14;
    for i in 0..line.len() - window_size + 1 {
        let token = &line[i..i + window_size];
        if is_marker(token) {
            return i + window_size;
        }
    }
    0
}
