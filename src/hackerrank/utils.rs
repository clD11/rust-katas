use std::io;

/// quick and dirty read lines functions for hackerrank

pub fn useful() {
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).expect("Fail");

    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Fail");
    message.truncate(message.len() - 1);

    if message.ends_with('\n') {
        message.pop();
    }

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail");
}

fn remove_newline(line: &str) -> String {
    line.replace("\n", "")
}

pub fn read_str_lines(n: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for _i in 0..n {
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

    let mut input = String::new();
    for _i in 0..n {
        io::stdin().read_line(&mut input).expect("Fail");
    }
}

fn read_u8() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Fail");
}

pub fn read_num() -> u32 {
    let mut str_num = String::new();
    io::stdin().read_line(&mut str_num).expect("Fail");
    str_num.trim().parse().unwrap()
}

pub fn read_vec() -> Vec<String> {
    let mut str_vec = String::new();
    io::stdin().read_line(&mut str_vec).expect("Fail");
    str_vec
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<String>>()
}
