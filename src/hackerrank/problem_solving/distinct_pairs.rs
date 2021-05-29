use std::collections::HashMap;
use std::collections::HashSet;

pub fn sum(nums: &[u32], target: u32) -> u32 {
    let mut pairs: HashMap<u32, HashSet<u32>> = HashMap::new();
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j {
                if nums[i] + nums[j] == target {
                    let lowest = if nums[i] < nums[j] { nums[i] } else { nums[j] };
                    let highest = if nums[i] > nums[j] { nums[i] } else { nums[j] };
                    pairs.entry(lowest).or_default().insert(highest);
                }
            }
        }
    }
    let mut count = 0;
    count += pairs.values().enumerate().len() as u32;
    count
}

#[test]
fn should_return_distinct_pairs_1() {
    let nums: &[u32; 9] = &[5, 7, 9, 13, 11, 6, 6, 3, 3];
    let actual = sum(nums, 12);
    assert_eq!(actual, 3);
}

#[test]
fn should_return_distinct_pairs_2() {
    let nums: &[u32; 6] = &[1, 1, 2, 45, 46, 46];
    let actual = sum(nums, 47);
    assert_eq!(actual, 2);
}
