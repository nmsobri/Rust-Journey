fn add_one(x: u32) -> u32 {
    return x + 1;
}

fn add_twice(f: fn(u32) -> u32, arg: u32) -> u32 {
    f(arg) + f(arg)
}

fn return_closure() -> Box<Fn(u32) -> u32> {
    return Box::new(|x| x + 1);
}

fn main() {
//    let a = add_twice(add_one, 5);
//
//    println!("{}", a);

    let a = return_closure();
    println!("{}", a(5));
}
