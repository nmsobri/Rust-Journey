use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //Demonstrating multiple mutability reference
    //Normally Rust wont allow multiple mutability
    let a: Rc<RefCell<u32>> = Rc::new(RefCell::new(9));

    let b = Rc::clone(&a);

    let c = Rc::clone(&a);

    println!("a is {}", *a.borrow());
    println!("b is {}", *b.borrow());
    println!("c is {}", *c.borrow());

    *a.borrow_mut() = 99;

    println!("");
    println!("a is {}", *a.borrow());
    println!("b is {}", *b.borrow());
    println!("c is {}", *c.borrow());

    *b.borrow_mut() = 88;

    println!("");
    println!("a is {}", *a.borrow());
    println!("b is {}", *b.borrow());
    println!("c is {}", *c.borrow());

    *c.borrow_mut() = 77;

    println!("");
    println!("a is {}", *a.borrow());
    println!("b is {}", *b.borrow());
    println!("c is {}", *c.borrow());
}
