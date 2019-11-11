struct User {
    name: String,
    age: i8,
    email: String,
}

struct Color(i32, i32, i32);

fn main() {
    let user = User {
        name: String::from("Sobri"),
        age: 38,
        email: String::from("slier81@gmail.com"),
    };

    println!("{:?}", user.name);

    let color = Color(100, 255, 250);
    println!("{}", color.0);
}
