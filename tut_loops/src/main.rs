fn main() {
    let mut num = 3;

    while num > 0 {
        println!("Num is {}", num);
        num = num - 1;
    }

    println!("LIFTOFF");

    let arr = [1, 3, 5];

    for i in arr.iter() {
        println!("elem is {}", i);
    }

    for i in (1..4).rev() {
        println!("{}!", i);
    }

    println!("LIFTOFF!!!");
}
