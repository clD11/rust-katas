use std::collections::HashSet;

// could do in place without the target message to save space
fn count_difference(input: &str) -> u32 {
    let message: &[u8] = input.as_bytes();

    let sos =  message.len() / 3;
    if sos == 0 {
        return 0;
    }

    // hackerrank will fail unstable so use "SOS"..repeat(sos).into_str();
    let target_message: Vec<u8> = [83, 79, 83].repeat(sos);

    let mut differences: u32 = 0;
    for index in 0..message.len() {
        if message[index] != target_message[index] {
            differences += 1;
        }
    }

    differences
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_no_difference_invalid() {
        let input: &str = "SO";
        let actual: u32 = count_difference(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_return_no_difference() {
        let input: &str = "SOSSOSSOS";
        let actual: u32 = count_difference(input);
        assert_eq!(actual, 0);
    }

    #[test]
    fn should_return_one_difference() {
        let input: &str = "SOSSOSSOT";
        let actual: u32 = count_difference(input);
        assert_eq!(actual, 1);
    }

    #[test]
    fn should_return_three_difference() {
        let input: &str = "SOSSPSSQSSOR";
        let actual: u32 = count_difference(input);
        assert_eq!(actual, 3);
    }
}
