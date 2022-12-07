use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub fn solve(fp: &str) -> i32 {
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

fn generate_range(borders: Vec<&str>) -> Vec<i32> {
    let mut range = vec![];
    let start: i32 = borders[0].parse::<i32>().unwrap();
    let stop: i32 = borders[1].parse::<i32>().unwrap() + 1;
    for i in start..stop {
        range.push(i as i32)
    }
    range
}

fn is_subset(range1: &Vec<i32>, range2: &Vec<i32>) -> bool {
    range2.iter().all(|item| range1.contains(item))
}

fn process_list(items: Vec<String>) -> i32 {
    let mut ranges1 = vec![];
    let mut ranges2 = vec![];
    for item in items.iter() {
        let range_defs = item.split(",").collect::<Vec<&str>>();
        let range_split = range_defs
            .iter()
            .map(|x| x.split("-").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        ranges1.push(generate_range(range_split[0].clone()));
        ranges2.push(generate_range(range_split[1].clone()));
    }

    let mut counter = 0;
    for (r1, r2) in ranges1.iter().zip(ranges2.iter()) {
        if is_subset(r1, r2) || is_subset(r2, r1) {
            counter += 1
        }
    }
    counter
}
