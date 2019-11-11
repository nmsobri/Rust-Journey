pub struct Guess {
    x: i32
}

impl Guess {
    pub fn new(_x: i32) -> Self {
        if _x < 1 {
            panic!("Value should be greater than 1");
        } else if _x > 100 {
            panic!("Value should be less than 100")
        }

        Self {
            x: _x
        }
    }
}

pub struct Rect {
    x: i32,
    y: i32,
}

impl Rect {
    fn new(_x: i32, _y: i32) -> Self {
        Self { x: _x, y: _y }
    }

    fn can_hold(&self, other: &Rect) -> bool {
        return self.x > other.x && self.y > other.y;
    }
}

pub fn add_two(x: i32) -> i32 {
    return x + 2;
}

pub fn greeting(name: &str) -> String { format!("Hello!") }

pub fn prints_and_return10(x: i32) -> i32 {
    println!("I got {}", x);
    10
}


#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
//    fn exploration() {
//        assert_eq!(2 + 2, 4);
//    }
//
//    #[test]
//    fn another() {
//        panic!("Foobar");
//    }
//
//    #[test]
//    fn can_hold() {
//        let r1 = Rect::new(5, 7);
//        let r2 = Rect::new(3, 2);
//
//        assert!(r1.can_hold(&r2));
//    }
//
//    #[test]
//    fn cannot_hold() {
//        let r1 = Rect::new(5, 7);
//        let r2 = Rect::new(3, 2);
//
//        assert!(!r2.can_hold(&r1));
//    }
//
//    #[test]
//    fn add_two() {
//        assert_eq!(super::add_two(2), 5);
//    }
//
//    #[test]
//    fn add_two_custom_msg() {
//        let result = super::add_two(2);
//        assert!(result == 6, "Result return is fooooooooooooooo{}", result);
//    }
//
//    #[test]
//    fn greeting_contains_name() {
//        let result = super::greeting("Carol");
//        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
//    }
//
//    #[test]
//    #[should_panic(expected = "Value foo")]
//    fn invalid_guess_should_panic() {
//        let guess = Guess::new(500);
//    }
//    #[test]
//    fn this_will_pass() {
//        assert_eq!(super::prints_and_return10(4), 10);
//    }
//
//    #[test]
//    fn this_will_fail() {
//        assert_eq!(super::prints_and_return10(7), 5);
//    }
//    #[test]
//    fn add2_to_add_two() {
//        assert_eq!(super::add_two(2), 4);
//    }
//
//    #[test]
//    fn add4_to_add_two() {
//        assert_eq!(super::add_two(4), 6);
//    }
//
//    #[test]
//    fn d100() {
//        assert_eq!(super::add_two(100), 102);
//    }
//
//    #[test]
//    #[ignore]
//    fn expensive_task() {}
}
