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

    for line in stdin.lock().lines() {
        let input: u32 = line.unwrap()
            .trim()
            .parse()
            .expect("Wanted a number");

        println!("Test {}", input);
    }
}