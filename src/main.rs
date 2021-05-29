#[macro_use]
extern crate lazy_static;
extern crate rand;

use hackerrank::utils::{read_num, read_str_lines};

mod hackerrank;
mod rust_book;

fn main() {
    let n = read_num();
    let input = read_str_lines(n);
    let result = hackerrank::problem_solving::gem_stones::find_intersection(input);
    println!("{:?}", result);
}
