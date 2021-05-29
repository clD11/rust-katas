fn mask_character(target: &str, input: &str) -> String {
    let mut characters: Vec<String> = input
        .split("")
        .filter(|c| *c != "")
        .map(|c| c.to_owned())
        .collect();

    for i in 0..characters.len() {
        if characters[i] == target {
            characters[i] = String::from("#");
        }
    }

    characters.join("")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_mask_character() {
        let target = String::from("z");
        let input = String::from("aaazaaa");
        let actual = mask_character(&target, &input);
        assert_eq!(actual, "aaa#aaa");
    }
}
