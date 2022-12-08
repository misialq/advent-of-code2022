use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub fn solve(fp: &str) -> String {
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

fn extract_piles(crates: &mut Vec<Vec<String>>) -> Vec<Vec<char>> {
    let no_piles = crates[0].len();
    let mut piles = vec![vec![]; no_piles];
    for row in crates.iter().rev() {
        for (j, _crate) in row.iter().enumerate() {
            if _crate != "" {
                piles[j].push(_crate.chars().nth(1).unwrap_or_else(|| ' '))
            }
        }
    }
    piles
}

fn extract_moves(moves: &mut Vec<Vec<&str>>) -> Vec<(i32, i32, i32)> {
    let moves = moves
        .iter()
        .map(|x| {
            (
                x[0].parse::<i32>().unwrap(),
                x[2].parse::<i32>().unwrap(),
                x[4].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32, i32)>>();
    moves
}

fn process_list(lines: Vec<String>) -> String {
    let mut moves = vec![];
    let mut crates = vec![];
    for line in lines.iter() {
        if line.contains("[") {
            crates.push(
                line.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect::<String>().trim_end().to_owned())
                    .collect::<Vec<String>>(),
            );
        } else if line.contains("move") {
            moves.push(line[5..].split(" ").collect::<Vec<&str>>())
        }
    }

    let moves = extract_moves(&mut moves);
    let piles = &mut extract_piles(&mut crates);

    for (item_count, from, to) in moves.iter() {
        let _from = *from as usize - 1;
        let _to = *to as usize - 1;
        let new_pile = &mut piles[_to].clone();
        for _ in 0..*item_count as usize {
            let current_len = &piles[_from].len();
            let current_item = piles[_from][current_len - 1];
            new_pile.push(current_item);
            piles[_from] = piles[_from][..current_len - 1].to_vec();
            piles[_to] = new_pile.to_vec();
        }
    }

    let mut tops = "".to_owned();
    for pile in piles {
        if pile.len() > 0 {
            tops.push_str(&pile[pile.len() - 1].to_string())
        }
    }
    tops
}
