use array_tool::vec::Intersect;
use std::collections::HashMap;
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

fn process_list(backpacks: Vec<String>) -> i32 {
    let mut letters = "abcdefghijklmnopqrstuvwxyz".to_owned();
    letters.push_str(&letters.to_uppercase());

    let mut priorities = [0; 52];
    for (i, v) in priorities.iter_mut().enumerate() {
        *v = i + 1 as usize
    }

    let mut priorities_map = HashMap::new();
    let chars = letters.chars().collect::<Vec<char>>();
    for (i, letter) in chars.iter().enumerate() {
        priorities_map.insert(letter, priorities[i]);
    }

    let mut common_items: Vec<char> = vec![];
    let bp_groups = backpacks
        .chunks(3)
        .map(|s| s.into())
        .collect::<Vec<Vec<String>>>();
    for bp_group in bp_groups.iter() {
        let bps = bp_group
            .iter()
            .map(|g| g.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let common_item = bps[0].intersect(bps[1].to_vec()).intersect(bps[2].to_vec());
        common_items.push(common_item[0])
    }

    let item_priorities = common_items
        .iter()
        .map(|x| priorities_map.get(x).unwrap())
        .collect::<Vec<&usize>>();
    item_priorities.iter().map(|x| **x as i32).sum()
}
