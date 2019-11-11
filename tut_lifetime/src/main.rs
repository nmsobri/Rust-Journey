use std::fmt::Display;

struct Point<T> { x: T, y: T }

fn longest<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("{}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let p: Point<i32> = Point { x: 3, y: 5 };
    let x = "Hello world mars";
    let y = "foo";

    let result = longest(x, y, "Yehaaaaaaaaaaaaaaaaaaaaa");
    println!("result is {}", result);
}

//struct Nodes<'a> {
//    left: &'a str,
//}

//impl<'a> Nodes<'a> {
//    fn traverse(&self, x: &str) -> &str {
//        return self.left;
//    }
//}

//struct Thing {
//    x: String,
//    y: String,
//}
//
//impl Thing {
//    fn new(_x: String, _y: String) -> Self {
//        Self {
//            x: _x,
//            y: _y,
//        }
//    }
//
//    fn c<'a>(&'a self) -> &'a str {
//        self.x.as_str()
//    }
//}
//
//fn main() {
//    let t = {
//        let u = Thing::new("hello".to_string(), "world".to_string());
//
//        u.c()
//    };
//
//    println!("t is {:#?}", t);
//}

//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}

//fn main() {
//    let n = Nodes {
//        left: "Hello",
//    };
//}

//fn main() {
//    let x = String::from("Hello world");
//    let j = &x;
//    let result;
//    {
//        let y = String::from("foo");
//        result = longest(j, &y);
//    }
//    println!("result is '{}'", result);
//}

//fn main() {
//    {
//        // Make a `string` literal and print it:
//        let static_string = "I'm in read-only memory";
//        println!("static_string: {}", static_string);
//
//        // When `static_string` goes out of scope, the reference
//        // can no longer be used, but the data remains in the binary.
//    }
//
//    println!("{}", static_string);
//}

