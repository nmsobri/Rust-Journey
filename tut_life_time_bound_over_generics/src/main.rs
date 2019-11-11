trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item> {
        unimplemented!()
    }
}

struct Counter {}

//impl Iterator<String> for Counter {}

//impl Iterator<u32> for Counter {}

//impl<T> Iterator<T> for Counter {}

impl Iterator for Counter {
    type Item = String;
}

fn main() {
    println!("Hello, world!");
}
