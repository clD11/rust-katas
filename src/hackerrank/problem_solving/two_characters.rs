use std::collections::{HashMap, HashSet};
use std::ops::Index;

fn two_characters(input: &str) -> i32 {

    if input.chars().count() <= 1 { return 0; }

    let mut char_matrix: [[u8; 26]; 26] = [[0; 26]; 26];
    let mut count_matrix: [[i32; 26]; 26] = [[0; 26]; 26];;

    for c in input.as_bytes() {

        let mut index: usize = (c - 97) as usize;

        for i in 0..26 {
            if char_matrix[index][i] == *c {
                count_matrix[index][i] = -1;
            }
            if count_matrix[index][i] != -1 {
                char_matrix[index][i] = *c;
                count_matrix[index][i] += 1;
            }
            if char_matrix[i][index] == *c {
                count_matrix[i][index] = -1;
            }
            if count_matrix[i][index] != -1 {
                char_matrix[i][index] = *c;
                count_matrix[i][index] += 1;
            }
        }
    }
    let mut highest: i32 = 0;
    for i in 0..26 {
        for j in 0..26 {
            if count_matrix[i][j] > highest {
                highest = count_matrix[i][j];
            }
        }
    }
    highest
}

#[cfg(test)]
mod tests {

    /// Remove characters until the string is made up of any two alternating characters.
    /// When you choose a character to remove, all instances of that character must be removed.
    /// Determine the longest string possible that contains just two alternating letters.

    use super::*;

    #[test]
    fn should_zero_when_one_character() {
        let input: &str = "a";
        let actual: i32 = two_characters(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_zero_alternating() {
        let input: &str = "aababa";
        let actual: i32 = two_characters(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_return_alternating() {
        let input: &str = "beabeefeab";
        let actual: i32 = two_characters(input);
        assert_eq!(actual, 5);
    }

    #[test]
    fn should_return_alternating_long() {
        let input: &str = "asdcbsdcagfsdbgdfanfghbsfdab";
        let actual: i32 = two_characters(input);
        assert_eq!(actual, 8);
    }

}