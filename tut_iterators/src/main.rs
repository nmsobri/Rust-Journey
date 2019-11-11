use std::thread::sleep;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(_size: u32, _style: String) -> Self {
        Self {
            size: _size,
            style: _style,
        }
    }
}

fn my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| {
        s.size == size
    }).collect()
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Self {
        Self {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            return Some(self.count);
        } else {
            return None;
        }
    }
}

fn main() {
    let mut a = Counter::new();
    let b: u32 = a.sum();

    println!("b is {}", b);
//    let x = vec![1, 2, 3, 5];
//
//    for i in x.iter() {
//        println!("i is {}", i);
//    }
//
//    let y: u32 = x.iter().sum();
//    println!("y is {}", y);
//
//    let z: Vec<_> = x.iter().map(|x| x + 1).collect();
//
//    println!("z is {:#?}", z);
}

#[test]
fn find_my_size() {
    let shoes = vec![
        Shoe::new(10, "sandal".to_string()),
        Shoe::new(13, "fipper".to_string()),
        Shoe::new(10, "sneaker".to_string())
    ];


    assert_eq!(my_size(shoes, 13), vec![Shoe::new(13, "fipper".to_string())])
}
