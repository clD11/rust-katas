#[macro_use]
extern crate lazy_static;
extern crate rand;

mod hackerrank;
mod rust_book;

use hackerrank::utils::{read_num, read_str_lines};
use std::io;
use std::io::BufRead;

fn main() {
    let n = read_num();
    let input = read_str_lines(n);
    let result = hackerrank::problem_solving::gem_stones::find_intersection(input);
    println!("{:?}", result);
}
