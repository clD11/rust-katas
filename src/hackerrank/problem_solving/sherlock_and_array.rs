use super::super::utils::*;

fn balanced_sums_count(arr: &Vec<u32>) -> &str {
    let total: u32 = arr.iter().sum();
    let mut sum: u32 = 0;
    for i in arr {
        if total - sum - *i == sum {
            return "YES";
        }
        sum += *i;
    }
    "NO"
}

fn balanced_sums(arr: &Vec<u32>) -> &str {

    if arr.len() == 1 {
        return "YES";
    }

    let mut gt_zero = 0;
    for i in arr {
        if *i > 0 {
            gt_zero += 1;
        }
    }

    if gt_zero <= 1 {
        return "YES";
    }

    let mut pos_left: usize = 0;
    let mut pos_right: usize = arr.len() - 1;

    let mut left_count: u32 = arr[pos_left];;
    let mut right_count: u32 = arr[pos_right];

    while pos_left < pos_right {

        if left_count > right_count {
            pos_right -= 1;
            right_count += arr[pos_right];
        }

        if left_count < right_count {
            pos_left += 1;
            left_count += arr[pos_left];
        }

        if left_count == right_count {
            if pos_right - (pos_left + 1) == 1 {
                return "YES";
            } else {
                pos_right -= 1;
                pos_left += 1;
                right_count += arr[pos_right];
                left_count += arr[pos_left];
            }

        }

    }

    "NO"
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_find_number_even_input() {
        let input: Vec<u32> = vec![5, 6, 8, 11];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_number_even_input_h2() {
        let input: Vec<u32> = vec![1, 1, 4, 1, 1];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_number_odd_input() {
        let input: Vec<u32> = vec![1, 1, 5, 6, 8, 11, 2];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_not_find_number_even_input() {
        let input: Vec<u32> = vec![1, 1, 5, 6, 8, 11, 2, 54];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_find_number_odd_input_11() {
        let input: Vec<u32> = vec![1, 1, 5, 6, 8, 11, 2];
        let actual: &str = balanced_sums_count(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_not_find_number_even_input_22() {
        let input: Vec<u32> = vec![1, 1, 5, 6, 8, 11, 2, 54];
        let actual: &str = balanced_sums_count(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_not_find_number_odd_input() {
        let input: Vec<u32> = vec![1, 1, 5, 6, 8, 11, 2, 54, 96];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_not_find_number_odd_input_middle() {
        let input: Vec<u32> = vec![1, 1, 1, 9, 8, 4 ];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_find_number_end_input() {
        let input: Vec<u32> = vec![1, 2, 3];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_find_number_zero_input_1() {
        let input: Vec<u32> = vec![2, 0, 0, 0];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_number_zero_input_2() {
        let input: Vec<u32> = vec![0, 0, 2, 0];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_number_single_input() {
        let input: Vec<u32> = vec![2];
        let actual: &str = balanced_sums(&input);
        assert_eq!(actual, "YES");
    }

    #[test]
    fn should_find_missing_numbers_h1() {
        let input: Vec<u32> = vec![75, 26, 45, 72, 81, 47, 29, 97, 2, 75, 25, 82, 84, 17, 56, 32, 2, 28, 37, 57, 39, 18, 11, 79, 6, 40, 68, 68, 16, 40, 63, 93, 49, 91, 10, 55, 68, 31, 80, 57, 18, 34, 28, 76, 55, 21, 80, 22, 45, 11, 67, 67, 74, 91, 4, 35, 34, 65, 80, 21, 95, 1, 52, 25, 31, 2, 53, 96, 22, 89, 99, 7, 66, 32, 2, 68, 33, 75, 92, 84, 10, 94, 28, 54, 12, 9, 80, 43, 21, 51, 92, 20, 97, 7, 25, 67, 17, 38, 100, 86];
        let actual: &str = balanced_sums_count(&input);
        assert_eq!(actual, "NO");
    }

    #[test]
    fn should_find_number_zero_input_11() {
        let input: Vec<u32> = vec![2, 0, 0, 0];
        let actual: &str = balanced_sums_count(&input);
        assert_eq!(actual, "YES");
    }

}