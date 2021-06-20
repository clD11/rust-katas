use std::str::FromStr;

fn find_beautiful_number(input: &str) -> String {
    let mut digits: usize = 1;
    let mut start_index: usize = 0;
    let mut end_index: usize = digits;

    loop {
        let next_value: Vec<(usize, &str)> = input
            .match_indices(
                &(u32::from_str(&input[start_index..end_index]).unwrap() + 1).to_string(),
            )
            .collect();

        if !next_value.is_empty() && next_value.get(0).unwrap().0 == end_index {
            start_index = next_value.get(0).unwrap().0;
            end_index = start_index + next_value.get(0).unwrap().1.len();
        } else {
            digits += 1;
            start_index = 0;
            end_index = digits;
        }

        if end_index >= input.len() - 1 {
            if !next_value.is_empty() {
                return format!("YES {}", &input[0..digits]);
            }
            return String::from("NO");
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_find_beautiful_number_nines() {
        let input: &str = "198199200";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 198");
    }

    #[test]
    fn should_find_beautiful_number_nine() {
        let input: &str = "91011";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 9");
    }

    #[test]
    fn should_find_beautiful_number_1() {
        let input: &str = "1234";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 1");
    }

    #[test]
    fn should_find_beautiful_number_2() {
        let input: &str = "99100";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 99");
    }

    #[test]
    fn should_find_beautiful_number_multiple_2() {
        let input: &str = "11121314";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 11");
    }

    #[test]
    fn should_find_beautiful_number_multiple_3() {
        let input: &str = "101102103104";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "YES 101");
    }

    #[test]
    fn should_not_find_beautiful_number_2() {
        let input: &str = "13";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_not_find_beautiful_number_1() {
        let input: &str = "1";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_not_find_beautiful_number_long() {
        let input: &str = "101103";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_not_find_beautiful_number() {
        let input: &str = "010203";
        let actual: String = find_beautiful_number(input);
        assert_eq!(actual, "NO");
    }
}
