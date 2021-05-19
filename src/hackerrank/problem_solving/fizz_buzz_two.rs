use std::borrow::Cow;

fn fizzbuzz_two() {
    for i in 1..101 {
        println!("{}", match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".into(),
            (0, _) => "Fizz".into(),
            (_, 0) => "Buzz".into(),
            _ => Cow::from(i.to_string()),
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_print_fizz_buzz() {
        fizzbuzz_two()
    }

}