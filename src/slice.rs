// Rust Book
// Here’s a small programming problem: write a function that takes a string and returns the first
// word it finds in that string. If the function doesn’t find a space in the string, the whole string
// must be one word, so the entire string should be returned.

use std::string::String;

pub fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn should_return_first_word() {
    let expected = String::from("first_word");
    let input = String::from("first_word nonsense");
    let actual = find_first_word(&input);
    assert_eq!(actual, expected);
}

#[test]
fn should_return_whole_string() {
    let expected = String::from("first_word_nonsense");
    let input = String::from("first_word_nonsense");
    let actual = find_first_word(&input);
    assert_eq!(actual, expected);
}