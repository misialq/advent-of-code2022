use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(fp: &str) -> i32 {
    let contents = read_file(fp);
    process_list(contents)
}

fn read_file(fp: &str) -> Vec<String> {
    println!("Reading in data from {}", fp);
    let file = File::open(fp).unwrap();
    let reader = BufReader::new(file);

    let mut contents = vec![];
    for line in reader.lines() {
        contents.push(line.unwrap());
    }
    contents
}

fn process_list(calories: Vec<String>) -> i32 {
    let mut elves = vec![];
    let mut current_elf = vec![];
    for (i, cal) in calories.iter().enumerate() {
        let cal_str = cal.as_str();
        if cal_str != "" {
            current_elf.push(cal_str.parse::<i32>().unwrap());
        } else {
            elves.push(current_elf.clone());
            current_elf = vec![];
        }

        if i == calories.len() - 1 {
            elves.push(current_elf.clone());
        }
    }

    let mut total_cals = elves.iter().map(|x| x.iter().sum()).collect::<Vec<i32>>();
    total_cals.sort_by(|a, b| b.cmp(a));
    total_cals[0..3].iter().sum()
}
