pub fn  divisible_sum_pairs(n: usize, k: u32, ar: &[u32]) -> u32 {
    let mut count: u32 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i] + ar[j]) % k == 0 {
                count = count + 1;
            }
        }
    }
    count
}

#[test]
fn should_return_number_of_divisible_sum_pairs() {
    let n = 6;
    let k = 3;
    let arr: &[u32; 6] = &[1, 3, 2, 6, 1, 2];
    let actual = divisible_sum_pairs(n, k, arr);
    assert_eq!(actual, 5);
}