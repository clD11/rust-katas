fn pairs_boxed() -> Box<[u32; 2]> {
    let arr: [u32; 2] = [1, 2];
    let mut boxed: Box<[u32; 2]> = Box::new(arr);
    boxed
}

fn pairs_vec() -> Vec<[u32; 2]> {
    let arr: [u32; 2] = [1, 2];
    let mut vector: Vec<[u32; 2]> = Vec::new();
    vector.push(arr);
    vector
}

#[test]
fn should_get_pairs_boxed() {
    let actual = pairs_boxed();
    assert_eq!(actual[0], 1);
    assert_eq!(actual[1], 2);
}

#[test]
fn should_get_pairs_vec() {
    let actual = pairs_vec();
    let expected: [u32; 2] = [1, 2];
    assert_eq!(actual[0][0], expected[0]);
    assert_eq!(actual[0][1], expected[1]);
}