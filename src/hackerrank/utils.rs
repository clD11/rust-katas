use std::io;
use std::io::{BufRead, Stdin};

pub fn read_num() -> u32 {
    let mut str_num = String::new();
    io::stdin().read_line(&mut str_num).expect("Fail");
    str_num.trim().parse().unwrap()
}

pub fn read_vec() -> Vec<u32> {
    let mut str_vec = String::new();
    io::stdin().read_line(&mut str_vec).expect("Fail");
    str_vec.split_whitespace()
        .map(|s | s.parse().unwrap())
        .collect::<Vec<u32>>()
}