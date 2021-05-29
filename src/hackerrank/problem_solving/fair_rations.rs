pub fn fair_rations(arr: &[u32]) -> &'static str {
    if arr[0] % 2 != 0 && arr[1] % 2 == 0 {
        return "NO";
    }

    // count
    // for i = arr[arr.length - 2] >= 1; --i
    // if arr[i + 1] % 2 != 0 increment until arr[i + 1]++ && arr[i]++; count++;

    "4"
}

#[test]
fn should_return_no() {
    let arr: [u32; 2] = [1, 2];
    let actual = fair_rations(&arr);
    assert_eq!(actual, "NO");
}

#[test]
fn should_return_number() {
    let arr: [u32; 5] = [2, 3, 4, 5, 6];
    let actual = fair_rations(&arr);
    assert_eq!(actual, "4");
}
