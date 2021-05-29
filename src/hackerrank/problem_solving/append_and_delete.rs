fn append_and_delete(initial: &str, desired: &str, operation: usize) -> &'static str {
    let shortest: usize = if initial.len() < desired.len() {
        initial.len()
    } else {
        desired.len()
    };
    let mut index = 0;
    for i in 0..shortest {
        if initial.as_bytes()[i] != desired.as_bytes()[i] {
            index = i;
            break;
        }
    }
    if initial.len() + desired.len() - index * 2 <= operation {
        return "yes";
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
        let operation: usize = 9;
        assert_eq!(append_and_delete(&initial, &desired, operation), "yes");
    }

    #[test]
    fn should_return_false_shorter() {
        let initial = String::from("ashley");
        let desired = String::from("ash");
        let operation: usize = 2;
        assert_eq!(append_and_delete(&initial, &desired, operation), "no");
    }

    #[test]
    fn should_return_true_equal() {
        let initial = String::from("aba");
        let desired = String::from("aba");
        let operation: usize = 7;
        assert_eq!(append_and_delete(&initial, &desired, operation), "yes");
    }
}
