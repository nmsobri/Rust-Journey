use std::fmt;
use std::fmt::{Display, Formatter};

struct Wrapper<T>(Vec<T>);

impl<T> Display for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", "hello")
    }
}

fn main() {
    let w = Wrapper(vec![2, 3, 5, 6]);
    let x = Wrapper(vec![true, false, true]);
    println!("{}", w);
    println!("{}", x);
}
