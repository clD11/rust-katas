fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

#[test]
fn should_print_fizzbuzz_to() {
    fizzbuzz_to(15);
}

#[test]
fn should_be_divisible() {
    let actual = is_divisible_by(6, 3);
    assert_eq!(actual, true);
}

#[test]
fn should_not_be_divisible() {
    let actual = is_divisible_by(6, 10);
    assert_eq!(actual, false);
}
