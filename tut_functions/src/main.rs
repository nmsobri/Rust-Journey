fn main() {
    println!("Hello, world!");
//    another_function(5, 99);
    let x = plus_one(12);
    println!("Value of x is {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
    println!("Another function");
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
