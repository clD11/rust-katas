use std::collections::HashSet;

pub fn find_intersection(arr: Vec<String>) -> usize {
    let mut char_sets: Vec<HashSet<char>> = Vec::new();
    arr.iter()
        .for_each(|s| char_sets.push(s.chars().collect::<HashSet<char>>()));
    let mut result = char_sets[0].clone();
    char_sets
        .iter()
        .skip(1)
        .for_each(|s| result = result.intersection(&s).cloned().collect());
    result.len()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_gem_stones() {
        let stones: Vec<String> = vec![
            String::from("abcdde"),
            String::from("baccd"),
            String::from("eeabg"),
        ];
        let actual = find_intersection(stones);
        assert_eq!(actual, 2)
    }

    #[test]
    fn should_return_no_gem_stones() {
        let stones: Vec<String> = vec![
            String::from("abcde"),
            String::from("fghi"),
            String::from("k"),
        ];
        let actual = find_intersection(stones);
        assert_eq!(actual, 0);
    }
}
