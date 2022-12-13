use ndarray::{arr3, Array2};
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

fn is_taller(tree: &i32, trees_left: Vec<i32>, trees_right: Vec<i32>) -> bool {
    let x = trees_left
        .iter()
        .filter(|_x| tree > _x)
        .collect::<Vec<&i32>>();
    let y = trees_right
        .iter()
        .filter(|_y| tree > _y)
        .collect::<Vec<&i32>>();
    println!("x: {:?}, y: {:?}", x, y);
    if (x.len() == trees_left.len()) | (y.len() == trees_right.len()) {
        true
    } else {
        false
    }
}

fn process_list(lines: Vec<String>) -> i32 {
    let mut grid = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|x| {
            x.iter()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // let new_grid = arr2(&grid);
    for r in grid.iter() {
        println!("{:?}", r);
    }
    println!();

    let edge_trees = 4 * (&grid.len() - 1);
    let mut visible_trees = 0;
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in grid.iter().enumerate() {
            if (0 < i && i < r.len() - 1) & (0 < j && j < c.len() - 1) {
                let tree = &grid[i][j];
                let trees_left = r[..j].to_vec();
                let trees_right = r[j + 1..].to_vec();
                let mut trees_top = vec![];
                let mut trees_bottom = vec![];
                for (x, t) in grid.iter().enumerate() {
                    if x < i {
                        trees_top.push(t[j])
                    } else if x > i {
                        trees_bottom.push(t[j])
                    }
                }
                println!(
                    "Tree: {:?}\tLeft: {:?}\tRight: {:?}",
                    tree, trees_left, trees_right
                );
                println!(
                    "Tree: {:?}\tTop: {:?}\tBottom: {:?}",
                    tree, trees_top, trees_bottom
                );

                if is_taller(tree, trees_left, trees_right)
                    | is_taller(tree, trees_top, trees_bottom)
                {
                    visible_trees += 1
                }
                println!();
            }
        }
    }
    (&visible_trees + &edge_trees) as i32
}
