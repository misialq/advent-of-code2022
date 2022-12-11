extern crate core;

use crate::day6::solve;
use std::env;

pub mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let result = solve(file_path);

    println!("Results: {:?}", result)
}
