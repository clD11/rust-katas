enum IpAddrKind {
    V4(String),
}

#[test]
fn test_V4() {
    let expected = String::from("127.0.0.1");
    let V4 = IpAddrKind::V4(String::from("127.0.0.1"));
    if let IpAddrKind::V4(actual) = V4 {
        assert_eq!(actual, expected);
    }
}
