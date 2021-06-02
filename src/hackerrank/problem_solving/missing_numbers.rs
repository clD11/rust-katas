use std::io;

use std::collections::HashMap;
use std::collections::HashSet;

fn missing_numbers(arr: &Vec<u32>, brr: &Vec<u32>) -> Vec<u32> {
    let mut arr_to_freq: HashMap<&u32, i32> = HashMap::new();
    for a in arr {
        arr_to_freq.entry(&a).and_modify(|i| *i += 1).or_insert(1);
    }

    let mut missing_numbers: HashSet<u32> = HashSet::new();

    for b in brr {
        let entry = arr_to_freq.get(&b);
        if entry.is_none() || *entry.unwrap() <= 0 {
            missing_numbers.insert(b.clone());
        } else {
            arr_to_freq.entry(&b).and_modify(|v| *v -= 1);
        }
    }

    let mut missing_numbers_ordered = missing_numbers.into_iter().collect::<Vec<u32>>();
    missing_numbers_ordered.sort();
    missing_numbers_ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_missing_numbers_1() {
        let arr: Vec<u32> = vec![203, 204, 205, 206, 207, 208, 203, 204, 205, 206];
        let brr: Vec<u32> = vec![
            203, 204, 204, 205, 206, 207, 205, 208, 203, 206, 205, 206, 204,
        ];
        let actual: Vec<u32> = missing_numbers(&arr, &brr);

        let expected: Vec<u32> = vec![204, 205, 206];

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_missing_numbers_2() {
        let arr: Vec<u32> = vec![7, 2, 5, 3, 5, 3];
        let brr: Vec<u32> = vec![7, 2, 5, 4, 6, 3, 5, 3];
        let actual: Vec<u32> = missing_numbers(&arr, &brr);

        let expected: Vec<u32> = vec![4, 6];

        assert_eq!(actual, expected);
    }
}
