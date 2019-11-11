fn main() {
    const PLAYER_HEALTH: u32 = 100;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of PLAYER HEALTh is : {}", PLAYER_HEALTH);

    let mut spaces = "   ";
    let mut spaces = spaces.len();

    let x: bool = true;
    println!("Value of x is {}", x);

    let c: char = 'f';
    println!("Value of c is {}", c);

    let tup: (u32, f64, char) = (36, 98.7, 'f');
    println!("The value of tup is {:?}", tup);

    let (x, y, z) = tup;
    println!("Value for x is {}, y is {}, z is {}", x, y, z);


    let tup2 = ("hello", 56, 'k');
    println!("Value of tup2 is {:?}", tup2);

    println!("Value of first of tup2 is {}", tup2.0);
    println!("Value of second of tup2 is {}", tup2.1);
    println!("Value of third of tup2 is {}", tup2.2);

    let arr = [2, 5, 7, 9];
    println!("Value of arr is {:?}", arr);

    println!("Value of arr[99] is {}", arr[99]);
}