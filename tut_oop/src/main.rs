mod collection;

use collection::AverageCollection;

fn main() {
    println!("Hello, world!");

    let mut a = AverageCollection {
        list: vec![],
        average: 0.0,
    };

    a.average = 9.8;

    println!("{}", a.average);

    a.foo();
}
