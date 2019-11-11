use std::fmt;
use std::fmt::{Display, Formatter};

trait Foo {}

trait Outline: Display {
    fn outline(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: u32,
    y: u32,
}

impl Outline for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Foo for Point {}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.outline();
}
