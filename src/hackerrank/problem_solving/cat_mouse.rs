pub fn cat_and_mouse(x: i32, y: i32, z: i32) -> &'static str {
    let a = i32::abs(z - x);
    let b = i32::abs(z - y);

    if a == b {
        return "Mouse C";
    }

    if a < b {
        "Cat A"
    } else {
        "Cat B"
    }
}

#[test]
fn should_return_cat_a_catches_mouse() {
    let cat_a_pos = 2;
    let cat_b_pos = 1;
    let mouse_pos = 3;
    let actual = cat_and_mouse(cat_a_pos, cat_b_pos, mouse_pos);
    assert_eq!(actual, "Cat A");
}

#[test]
fn should_return_cat_b_catches_mouse() {
    let cat_a_pos = 1;
    let cat_b_pos = 4;
    let mouse_pos = 5;
    let actual = cat_and_mouse(cat_a_pos, cat_b_pos, mouse_pos);
    assert_eq!(actual, "Cat B");
}

#[test]
fn should_return_mouse_escapes() {
    let cat_a_pos = 1;
    let cat_b_pos = 3;
    let mouse_pos = 2;
    let actual = cat_and_mouse(cat_a_pos, cat_b_pos, mouse_pos);
    assert_eq!(actual, "Mouse C");
}
