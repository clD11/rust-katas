fn append_and_delete(initial: &str, desired: &str, operation: u32) -> &'static str {
    let mut start_index: usize = 0;
    while start_index < initial.len() &&
        start_index < desired.len() &&
        initial.as_bytes()[start_index] == desired.as_bytes()[start_index] {
        start_index += 1;
    }

    if initial.len() - start_index + desired.len() - start_index <= operation as usize {
        return "yes"
    }

    "no"
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_true_shorter() {
        let initial = String::from("hackerhappy");
        let desired = String::from("hackerrank");
        let operation: u32 = 9;
        assert_eq!(append_and_delete(&initial, &desired, operation), "yes");
    }

    #[test]
    fn should_return_false_shorter() {
        let initial = String::from("ashley");
        let desired = String::from("ash");
        let operation: u32 = 2;
        assert_eq!(append_and_delete(&initial, &desired, operation), "no");
    }

    #[test]
    fn should_return_true_equal() {
        let initial = String::from("aba");
        let desired = String::from("aba");
        let operation: u32 = 7;
        assert_eq!(append_and_delete(&initial, &desired, operation), "yes");
    }
}
