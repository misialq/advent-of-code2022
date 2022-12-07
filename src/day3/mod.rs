use array_tool::vec::Intersect;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator};

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
    for bp in backpacks.iter() {
        let bp_size = bp.len();
        let c1 = &bp[..bp_size / 2].chars().collect::<Vec<char>>();
        let c2 = &bp[bp_size / 2..].chars().collect::<Vec<char>>();
        let common_item = c1.intersect(c2.to_vec());
        common_items.push(common_item[0])
    }

    let item_priorities = common_items
        .iter()
        .map(|x| priorities_map.get(x).unwrap())
        .collect::<Vec<&usize>>();
    item_priorities.iter().map(|x| **x as i32).sum()
}
