/*
    Rust Book - struct, methods
*/

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // associated function
    fn square(d: i32) -> Rectangle {
        Rectangle {
            width: d,
            height: d
        }
    }

    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() >= rectangle.area()
    }
}

#[test]
fn calculate_area() {
    let rec = Rectangle {
        width: 3,
        height: 4
    };
    let expected = 12;
    let actual = rec.area();
    assert_eq!(actual, expected);
}

#[test]
fn can_hold() {
    let rec1 = Rectangle {
        width: 4,
        height: 4
    };

    let rec2 = Rectangle {
        width: 3,
        height: 4
    };

    let actual = rec1.can_hold(&rec2);
    assert!(actual);
}

#[test]
fn create_square() {
    let square = Rectangle::square(4);
    assert_eq!(square.height, 4);
    assert_eq!(square.width, 4);
    assert_eq!(square.area(), 16);
}