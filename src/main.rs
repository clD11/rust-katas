#[macro_use]
extern crate lazy_static;
extern crate rand;

mod hackerrank;
mod rust_book;

use std::io;
use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();

    let mut numberLines = String::new();
    stdin.read_line(&mut numberLines)
        .expect("Failed to read line");

    let lines_and_target: Vec<u32> = numberLines.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut input = String::new();
    stdin.read_line(&mut input)
        .expect("Failed to read line");

    let input_vec: Vec<u32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        println!("Test {}", input);

    let actual: u32 = hackerrank::problem_solving::pairs::find_pairs_to_target(input_vec, lines_and_target[1]);
}

// fn read() {
//     for line in stdin.lock().lines() {
//         let input: u32 = line.unwrap()
//             .trim()
//             .parse()
//             .expect("Wanted a number");
// }