fn make_anagram(a: &str, b: &str) -> u32 {
    let mut char_freq: [i32; 26] = [0; 26];

    for byte in a.bytes() {
        char_freq[(byte - 97) as usize] += 1;
    }

    for byte in b.bytes() {
        char_freq[(byte - 97) as usize] -= 1;
    }

    char_freq.iter().map(|&n| i32::abs(n) as u32).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_make_anagram_same_length() {
        let a = "cde";
        let b = "abc";
        let actual: u32 = make_anagram(a, b);
        assert_eq!(actual, 4);
    }

    #[test]
    fn should_make_anagram_one_longer() {
        let a = "abc";
        let b = "mnop";
        let actual: u32 = make_anagram(a, b);
        assert_eq!(actual, 7);
    }

    #[test]
    fn should_make_anagram_multiple_chars_same() {
        let a = "abc";
        let b = "aaaamnop";
        let actual: u32 = make_anagram(a, b);
        assert_eq!(actual, 9);
    }

    #[test]
    fn should_make_anagram_long_delete() {
        let a = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let b = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        let actual: u32 = make_anagram(a, b);
        assert_eq!(actual, 104);
    }
}
