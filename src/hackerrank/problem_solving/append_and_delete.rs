fn append_and_delete(initial: &str, desired: &str, operation: u32) -> &'static str {
    // let start_index: u32 = 0;
    // while n
    "yes"
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

    // #[test]
    // fn should_return_false_shorter() {
    //     let initial = String::from("ashley");
    //     let desired = String::from("ash");
    //     let operation: u32 = 2;
    //     assert_eq!(append_and_delete(&initial, &desired, operation), "no");
    // }
}
