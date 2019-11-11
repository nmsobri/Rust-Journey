fn main() {
    let condition: bool = false;

    let foo: i32 = if condition {
        55
    } else {
        "six"
    };

    println!("Value of foo is {}", foo);
}
