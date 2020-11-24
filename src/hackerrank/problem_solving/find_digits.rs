pub fn find_digits(n: u32) -> u32 {
    let mut result: u32 = 0;
    let mut sub: u32 = n;
    while sub > 0 {
        let div: u32 = sub % 10;
        if n % div == 0 {
            result = result + 1;
        }
        sub = sub / 10;
    }
    result
}

#[test]
fn should_return_divisor_true() {
    let expected = 2;
    let actual = find_digits(12);
    assert_eq!(actual, expected);
}