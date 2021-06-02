fn has_hackerrank(input: &str) -> &'static str {
    let mut pos = 0;
    for (index, char) in "hackerrank".char_indices() {
        if input.contains(char) && index >= pos {
            pos = index;
        } else {
            return "NO";
        }
    }
    "YES"
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_find_hackerrank() {
        let input: &str = "eehreiackerjrank";
        let actual = has_hackerrank(input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_hackerrank_long() {
        let input: &str = "hereiamstackerrank";
        let actual = has_hackerrank(input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_not_find_hackerrank() {
        let input: &str = "hackerworld";
        let actual = has_hackerrank(input);
        assert_eq!(actual, "NO");
    }
}
