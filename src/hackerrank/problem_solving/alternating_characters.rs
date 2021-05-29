fn count_number_deletions(input: String) -> u32 {
    let mut deletions = 0;
    let mut prev: char = ' ';
    for c in input.chars() {
        if c != prev {
            prev = c;
        } else {
            deletions += 1;
        }
    }
    deletions
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_delete_when_same() {
        let input: String = String::from("AAAA");
        let actual: u32 = count_number_deletions(input);
        assert_eq!(actual, 3);
    }

    #[test]
    fn should_delete_when_not_alternating() {
        let input: String = String::from("AAABBB");
        let actual: u32 = count_number_deletions(input);
        assert_eq!(actual, 4);
    }

    #[test]
    fn should_not_delete_when_alternating() {
        let input: String = String::from("ABABABAB");
        let actual: u32 = count_number_deletions(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_delete_when_alternating_end() {
        let input: String = String::from("ABABABAA");
        let actual: u32 = count_number_deletions(input);
        assert_eq!(actual, 1);
    }
}
