enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::*;
use std::rc::Rc;

//use std::ops::Deref;
//use std::alloc::handle_alloc_error;
//
//struct MyBox<T>(T);
//
//impl<T> MyBox<T> {
//    fn new(x: T) -> Self {
//        Self(x)
//    }
//}
//
//impl<T> Deref for MyBox<T> {
//    type Target = T;
//
//    fn deref(&self) -> &T {
//        &self.0
//    }
//}
//
//fn hello(s: &str) {
//    println!("Hello {}", s);
//}

struct SmartPointer<'a> {
    data: &'a str
}

impl<'a> SmartPointer<'a> {
    fn new(_data: &'a str) -> Self {
        Self { data: _data }
    }
}

impl<'a> Drop for SmartPointer<'a> {
    fn drop(&mut self) {
        println!("Dropping smart pointer {}", self.data);
    }
}

fn main() {
    let a = 5;
    let b = &mut a;
//    let a = Rc::new(Cons(5, Rc::new(Cons(4, Rc::new(Nil)))));
//    let b = Cons(1, Rc::clone(&a));
//    {
//        let c = Cons(2, Rc::clone(&a));
//        println!("number of reference is {}", Rc::strong_count(&a));
//    }
//
//    println!("number of reference is {}", Rc::strong_count(&a));
//    let a = SmartPointer::new("Smart A");
//    drop(a);
//    let b = SmartPointer::new("Smart B");
//
//    println!("Smart pointer created");

//    let a = Cons(1, Cons(2, Cons(3, Nil)));
//    let b = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

//    let a = 5;
//    let b = &a;
//
//    assert_eq!(5, a);
//    assert_eq!(5, *b);

//    let a = 3;
//    let b = Box::new(a);
//
//    assert_eq!(3, a);
//    assert_eq!(3, *b);

//    let a = 5;
//    let b = MyBox(a);
//
//    assert_eq!(5, a);
//    assert_eq!(5, *b);

//    hello("sobri");
//
//    let a = MyBox::new(String::from("Sobri Kamal"));
//    hello(&a);
}
