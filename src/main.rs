#[macro_use]
extern crate lazy_static;
extern crate rand;

mod hackerrank;
mod rust_book;

use std::io;
use std::io::{BufRead};

fn main() {
    hackerrank::problem_solving::missing_numbers::main();
}