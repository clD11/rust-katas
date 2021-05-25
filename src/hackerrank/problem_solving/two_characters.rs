use std::collections::{HashMap, HashSet};
use std::ops::Index;

fn find_alternating_characters(input: &str) -> u32 {
    let mut copy = String::from(input);
    loop {
        let mut c: char = find_consecutive(&copy);
        if c != ' ' {
            copy = copy.replace(c, "");
        } else {
            break;
        }
    }
    let mut freq: [u32; 26] = Default::default();
    for c in copy.chars() {
        let index: usize = c as usize - 97;
        freq[index] = freq[index] + 1
    }
    freq.sort();
    freq[24..26].iter().sum::<u32>()
}

fn find_consecutive(input: &str) -> char {
    let mut prev: char = ' ';
    for c in input.chars() {
        if c != prev {
            prev = c;
        } else {
            return c;
        }
    }
    ' '
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_zero_alternating() {
        let input: &str = "aababa";
        let actual: u32 = find_alternating_characters(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_return_alternating() {
        let input: &str = "beabeefeab";
        let actual: u32 = find_alternating_characters(input);
        assert_eq!(actual, 5);
    }

}