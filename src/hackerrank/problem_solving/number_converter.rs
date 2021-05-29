use std::collections::BTreeMap;

lazy_static! {
    static ref NUMBER_TO_NUMERAL: BTreeMap<u32, &'static str> = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (8, "VIII"),
        (7, "VII"),
        (6, "VI"),
        (5, "V"),
        (4, "IV"),
        (3, "III"),
        (2, "II"),
        (1, "I"),
    ]
    .iter()
    .copied()
    .collect();
}

pub fn to_roman_numeral(number: u32) -> String {
    let mut numeral = String::new();
    let mut remainder = number;
    for (key, value) in NUMBER_TO_NUMERAL.iter().rev() {
        while remainder >= *key {
            remainder -= *key;
            numeral.push_str(value);
        }
    }
    numeral
}

#[test]
fn should_convert_normal_numbers_to_roman_numerals_i() {
    let expected = "I";
    let actual = to_roman_numeral(1);
    assert_eq!(actual, expected);
}

#[test]
fn should_convert_normal_numbers_to_roman_numerals_iv() {
    let expected = "IV";
    let actual = to_roman_numeral(4);
    assert_eq!(actual, expected);
}

#[test]
fn should_convert_normal_numbers_to_roman_numerals_xi() {
    let expected = "XI";
    let actual = to_roman_numeral(11);
    assert_eq!(actual, expected);
}

#[test]
fn should_convert_normal_numbers_to_roman_numerals_cmlxxxviii() {
    let expected = "CMLXXXVIII";
    let actual = to_roman_numeral(988);
    assert_eq!(actual, expected);
}
