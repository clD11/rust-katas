use std::collections::HashMap;

pub fn find_pairs_to_target(input: Vec<u32>, target: u32) -> u32 {
    let mut numbers: HashMap<u32, bool> = HashMap::new();
    for i in &input {
        numbers.entry(*i).or_insert(true);
    }
    let mut number_of_pairs: u32= 0;
    let mut pair: u32 = 0;
    for i in input {
        if i >= target {
            pair = i - target;
            if numbers.contains_key(&pair) {
                number_of_pairs += 1;
            }
        }
    }
    number_of_pairs
}

fn pairs_boxed() -> Box<[u32; 2]> {
    let arr: [u32; 2] = [1, 2];
    let mut boxed: Box<[u32; 2]> = Box::new(arr);
    boxed
}

fn pairs_vec() -> Vec<[u32; 2]> {
    let arr: [u32; 2] = [1, 2];
    let mut vector: Vec<[u32; 2]> = Vec::new();
    vector.push(arr);
    vector
}

#[test]
fn should_find_pairs_to_target_value_one() {
    let input: Vec<u32> = vec!(1, 2, 3, 4);
    let actual = find_pairs_to_target(input, 1);
    assert_eq!(actual, 3);
}

#[test]
fn should_find_pairs_to_target_value_two() {
    let input: Vec<u32> = vec!(1, 5, 3, 4, 2);
    let actual = find_pairs_to_target(input, 2);
    assert_eq!(actual, 3);
}

#[test]
fn should_get_pairs_boxed() {
    let actual = pairs_boxed();
    assert_eq!(actual[0], 1);
    assert_eq!(actual[1], 2);
}

#[test]
fn should_get_pairs_vec() {
    let actual = pairs_vec();
    let expected: [u32; 2] = [1, 2];
    assert_eq!(actual[0][0], expected[0]);
    assert_eq!(actual[0][1], expected[1]);
}