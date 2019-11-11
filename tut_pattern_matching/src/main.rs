//#![allow(unused)]
//#![allow(dead_code)]

//fn coordinate((x, y): (i32, i32)) {
//    println!("Coordinate is, x: {}, y: {}", x, y);
//}

//fn foo(_: i32, y: i32) {
//    println!("y value is {}", y);
//}

fn main() {
    let a = 5;

    match a {
        val @ 1...100 => println!("Inside range, and the value is :{}", val),
        _ => println!("Not in a range"),
    }
//    let a = 4;
//    let y = true;
//
//    match a {
//        3 | 4 | 4 | 6 if y => println!("Value is {}", a),
//        _ => println!("Value is none")
//    }
//    let mut a = Some(String::from("Herror"));
//
//    match a {
//        Some(ref mut value) => *value = String::from("Yehaaaaaaaaaaaaaaaaaaaaaaaa"),
//        _ => println!("Found None")
//    }
//
//    println!("Value of a is {:?}", a);
    //    struct Point {
//        x: i32,
//        y: i32,
//        z: i32,
//    }
//
//    let a = Point { x: 3, y: 5, z: 7 };
//
//    let Point { .., x } = a;
//
//    println!("x is {}", x);
//
//
//    let a = (1, 3, 4, 5);
//
//    let (first, .., last) = a;
//    println!("first is {}", first);
//
//    println!("last is {}", last);
//    let a = Some(String::from("Hello"));
//    let _b = 7;
//
//    if let Some(_) = a {
//        println!("Got a value");
//    }
//
//    println!("a is {:#?}", a);
//    let mut setting = Some(999);
//    let new_setting = None;
//
//    match (setting, new_setting) {
//        (Some(_), Some(_)) => println!("You are not allowed to override setting"),
//        _ => setting = new_setting
//    }
//
//    println!("setting is {:#?}", setting);
//    foo(3, 5);
//    struct Point {
//        x: i32,
//        y: i32,
//    };
//
//    let points = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 5 }, Point { x: 10, y: -3 }, ];
//    let sum_of_squares: i32 = points.iter().map(|Point { x, y }| x * x + y * y).sum();
//
//    println!("square sum {}", sum_of_squares);

//    let a = vec![
//        Point { x: 1, y: 2 },
//        Point { x: 3, y: 4 },
//        Point { x: 5, y: 6 }
//    ];
//
//    let square_sum: i32 = a.iter().map(|Point { x, y }| {
//        return x * x + y * y;
//    }).sum();
//
//    println!("square sum {}", square_sum);
//    enum Message {
//        Quit,
//        Move(i32, i32),
//        Write(String),
//        ChangeColor(i32, i32, i32),
//    }
//
//    let a = Message::Write("hello world".to_string());
//
//    match a {
//        Message::Quit => println!("The Quit variant has no data"),
//        Message::Move(x, y) => println!("Moving direction at x {}, y {}", x, y),
//        Message::Write(text) => println!("Writing {}", text),
//        Message::ChangeColor(r, g, b) => println!("Change color to r {}, g {}, b{}", r, g, b),
//    }
//    struct Point {
//        x: i32,
//        y: i32,
//    }
//
//    let a = Point { x: 3, y: 0 };
//
//    match a {
//        Point { x, y: 0 } => println!("Literally on x axis"),
//        Point { x: 0, y } => println!("Literally on y axis"),
//        Point { x, y } => println!("Something else")
//    }
//    let Point { x: b, y: c } = a;
//
//    println!("b is {}", b);
//    println!("c is {}", c);
//
//    let Point { x, y } = a;
//
//    println!("x is {}", x);
//    println!("y is {}", y);
//    let a = 50;
//
//    match a {
//        1...10 => println!("in between 1 to 10"),
//        11...20 => println!("in between 11 to 20"),
//        _ => println!("something else")
//    }


//    let a = 8;
//
//    match a {
//        1 | 2 => println!("One or two"),
//        3 => println!("Its three"),
//        _ => println!("Default case")
//    }
//    if let Some(x) = Some(8) {
//        println!("foo");
//    }
//
//    if let x = 5 {
//        println!("bar");
//    }
//    let age: Option<i32> = None;
//
//    if let Some(umur) = age {
//        println!("Age is {}", umur);
//    }

//    let mut a = vec![1, 3, 5, 7, 9];
//
//    while let Some(x) = a.pop() {
//        println!("elem is {}", x);
//    }

//    let a = vec![1, 3, 5, 7, 9];
//
//    for (index, value) in a.iter().enumerate() {
//        println!("index: {}, value: {}", index, value);
//    }

//    let (x, y, _) = (1, 2, 3);
//    println!("x: {}, y: {}, ", x, y);
//
//    coordinate((2, 3));
}
