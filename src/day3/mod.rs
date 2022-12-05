use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Iterator, Zip};
use std::slice::Iter;

pub fn solve(fp: &str) -> i32 {
    let (player1, player2) = read_file(fp);
    process_list(player1, player2)
}

fn read_file(fp: &str) -> (Vec<char>, Vec<char>) {
    println!("Reading in data from {}", fp);
    let file = File::open(fp).unwrap();
    let reader = BufReader::new(file);

    let mut contents = vec![];
    for line in reader.lines() {
        let l = line.unwrap();
        contents.push(l);
    }

    let player1 = contents
        .iter()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Vec<char>>();
    let player2 = contents
        .iter()
        .map(|x| x.chars().last().unwrap())
        .collect::<Vec<char>>();

    (player1, player2)
}

fn is_winner(a: &char, b: &char) -> bool {
    if (a == &'A' && b == &'Z') || (a == &'C' && b == &'Y') || (a == &'B' && b == &'X') {
        true
    } else {
        false
    }
}

fn is_draw(a: &char, b: &char) -> bool {
    if (a == &'A' && b == &'X') || (a == &'B' && b == &'Y') || (a == &'C' && b == &'Z') {
        true
    } else {
        false
    }
}

fn pick_strategy<'a>(players_move: &'a char, desired_outcome: &'a char) -> &'a char {
    let strategies: HashMap<(&char, &char), &char> = HashMap::from([
        ((&'A', &'X'), &'Z'),
        ((&'A', &'Y'), &'X'),
        ((&'A', &'Z'), &'Y'),
        ((&'B', &'X'), &'X'),
        ((&'B', &'Y'), &'Y'),
        ((&'B', &'Z'), &'Z'),
        ((&'C', &'X'), &'Y'),
        ((&'C', &'Y'), &'Z'),
        ((&'C', &'Z'), &'X'),
    ]);
    strategies.get(&(players_move, desired_outcome)).unwrap()
}

fn process_list(p1: Vec<char>, p2: Vec<char>) -> i32 {
    let points: HashMap<char, i32> =
        HashMap::from([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);
    let mut p1_points = 0;
    let mut p2_points = 0;

    for (i, (m1, m2)) in p1.iter().zip(p2.iter()).enumerate() {
        let strategy = pick_strategy(m1, m2);
        p1_points += points.get(m1).unwrap();
        p2_points += points.get(strategy).unwrap();

        if is_winner(m1, strategy) {
            p1_points += 6
        } else if is_draw(m1, strategy) {
            p1_points += 3;
            p2_points += 3;
        } else {
            p2_points += 6
        }
    }
    p2_points
}
