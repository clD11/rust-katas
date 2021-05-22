use std::io;
use std::io::{BufRead, Stdin};

pub fn read_num() -> u32 {
    let mut str_num = String::new();
    io::stdin().read_line(&mut str_num).expect("Fail");
    str_num.trim().parse().unwrap()
}

pub fn read_vec() -> Vec<String> {
    let mut str_vec = String::new();
    io::stdin().read_line(&mut str_vec).expect("Fail");
    str_vec.split_whitespace()
        .map(|s | s.parse().unwrap())
        .collect::<Vec<String>>()
}

/// quick and dirty read lines for hackerrank
///
///# examples
/// ```
/// read_str_lines(3)
/// abc
/// zyx
/// fed
/// result: ["abc", "zyx", "fed"]
/// ```
pub fn read_str_lines(n: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 0..n {
        let mut str_vec = String::new();
        io::stdin().read_line(&mut str_vec).expect("Fail");
        str_vec.truncate(str_vec.len() - 1);
        result.push(str_vec);
    }
    result
}

fn read_lines() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail");
    let n: u32 = num.trim().parse().unwrap();

    for i in 0..n {
        let mut str_vec = String::new();
        io::stdin().read_line(&mut str_vec).expect("Fail");
        str_vec.truncate(str_vec.len() - 1);
    }
}