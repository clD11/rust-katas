fn mask_character(target: &str, input: &str) -> String {
    String::from("aaaaaa")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_mask_character() {
        let target = String::from("z");
        let input = String::from("aaazaaa");
        let actual = mask_character(&target, &input);
        assert_eq!(actual, "aaaaaa");
    }

}