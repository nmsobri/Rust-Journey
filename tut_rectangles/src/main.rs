#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        return Rectangle {
            width: w,
            height: h,
        };
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(200, 10);

    println!("the area of rectangles is {} square pixels", rect.area());
    println!("rect can hold rect2? : {}", rect.can_hold(&rect2));
}


