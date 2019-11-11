use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Point3 {
    x: u32,
    y: u32,
    z: u32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add<Point> for Point3 {
    type Output = Self;

    fn add(self, other: Point) -> Self::Output {
        return Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: 0,
        };
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}


fn main() {
//    let p1 = Point { x: 1, y: 2 };
//    let p2 = Point { x: 3, y: 4 };
//
//    let p3 = p1 + p2;
//
//    println!("{:#?}", p3);

    let p3 = Point3 {
        x: 1,
        y: 2,
        z: 3,
    };

    let pp3 = Point3 {
        x: 1,
        y: 2,
        z: 3,
    };

    let p1 = Point {
        x: 1,
        y: 2,
    };

    let p33 = Point3 {
        x: 3,
        y: 4,
        z: 5,
    };

    let p4 = p3 + p1;

    println!("{:#?}", p4);

    let p5 = pp3 + p33;

    println!("{:#?}", p5);
}
