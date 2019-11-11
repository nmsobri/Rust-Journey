use std::thread;
use std::time::Duration;

struct Cacher<S: Copy, T: Fn(S) -> S> {
    value: Option<S>,
    calculation: T,
}

impl<S, T> Cacher<S, T> where S: Copy, T: Fn(S) -> S {
    fn new(cb: T) -> Self {
        return Self {
            value: None,
            calculation: cb,
        };
    }

    fn value(&mut self, val: S) -> S {
        match &self.value {
            Some(v) => (*v).clone(),
            None => {
                let v = (self.calculation)(val);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
//    let mut foo = Cacher::new(|num| {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    });
//
//    let res = foo.value(3);
//    println!("res is {}", res);
//
//    let res2 = foo.value("Foooooo");
//    println!("res2 is {}", res2);
//
////    let res3 = foo.value("hello");
////    println!("res2 is {}", res3);
//
//    let mut bar = Cacher::new(|num| {
//        println!("Calculatin bar slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    });
//
//    let blah = bar.value("hello");
//    println!("blah is {}", blah);

//    let x = 4;
//
//    let eq_to_x = |num| num == x;
//
//    let res = eq_to_x(4);
//
//    println!("result is {}", res);

    let x: Vec<u32> = vec![1, 2, 4, 5];

    let mv_fn = move |z: Vec<u32>| {
        z == x
    };

    println!("vec is {:#?}", x);
}
