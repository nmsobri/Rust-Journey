#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V5(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

enum Coins {
    Penny(u32),
    Nickel(u32),
    Dime(u32),
    Quarter(u32),
}

fn value_in_cents(c: &Coins) -> u32 {
    return match c {
        Coins::Penny(val) => val.clone(),
        Coins::Nickel(val) => val.clone(),
        Coins::Dime(val) => val.clone(),
        Coins::Quarter(val) => val.clone()
    };
}

fn plus_one(val: Option<u32>) -> Option<u32> {
    return match val {
        Option::None => None,
        Option::Some(val) => Some(val + 1)
    };
}

fn main() {
    let bar: Option<i8> = None;

    if let Some(i) = bar {
        println!("bar has some value");
    } else {
        println!("bar is NOne");
    }


    let x = Option::Some(5);
    let six = plus_one(x);
    let foo = plus_one(Option::None);

    println!("x is {:?}", x);
    println!("six is {:?}", six);
    println!("foo is {:?}", foo);

    println!("######################");
    let coin = Coins::Quarter(95);
    println!("Coin is {}", value_in_cents(&coin));
    let msg = Message::Write(String::from("Hello Foo"));
    msg.call();
}
