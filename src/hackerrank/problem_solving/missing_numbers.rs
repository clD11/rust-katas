use std::io;
use std::io::{BufRead, Stdin};

use std::collections::HashMap;
use std::collections::HashSet;

fn missing_numbers(arr: &Vec<u32>, brr: &Vec<u32>) -> HashSet<u32> {
    let mut arr_to_freq: HashMap<&u32, i32> = HashMap::new();
    for a in arr {
        arr_to_freq.entry(&a)
            .and_modify(|i | { *i += 1 })
            .or_insert(1);
    }

    let mut missing_numbers: HashSet<u32> = HashSet::new();

    for b in brr {
        let entry = arr_to_freq.get(&b);
        if entry.is_none() || *entry.unwrap() <= 0 {
            missing_numbers.insert(b.clone());
        } else {
            arr_to_freq.entry(&b).and_modify(|v | *v -= 1);
        }
    }

    missing_numbers
}

fn read_num() -> u32 {
    let mut str_num = String::new();
    io::stdin().read_line(&mut str_num).expect("Fail");
    str_num.trim().parse().unwrap()
}

fn read_vec() -> Vec<u32> {
    let mut str_vec = String::new();
    io::stdin().read_line(&mut str_vec).expect("Fail");
    str_vec.split_whitespace()
        .map(|s | s.parse().unwrap())
        .collect::<Vec<u32>>()
}

pub fn main() {
    read_num();
    let arr = read_vec();
    read_num();
    let brr = read_vec();
    println!("{:?}", missing_numbers(&arr, &brr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_missing_numbers_1() {
        let arr: Vec<u32> = vec![203, 204, 205, 206, 207, 208, 203, 204, 205, 206];
        let brr: Vec<u32> = vec![203, 204, 204, 205, 206, 207, 205, 208, 203, 206, 205, 206, 204];
        let actual: HashSet<u32> = missing_numbers(&arr, &brr);

        let mut expected: HashSet<u32> = HashSet::new();
        expected.insert(204);
        expected.insert(205);
        expected.insert(206);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_missing_numbers_2() {
        let arr: Vec<u32> = vec![7, 2, 5, 3, 5, 3];
        let brr: Vec<u32> = vec![7, 2, 5, 4, 6, 3, 5, 3];
        let actual: HashSet<u32> = missing_numbers(&arr, &brr);

        let mut expected: HashSet<u32> = HashSet::new();
        expected.insert(4);
        expected.insert(6);

        assert_eq!(actual, expected);
    }

}