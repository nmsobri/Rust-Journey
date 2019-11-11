extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

#[derive(Debug)]
struct Guess {
    value: u32
}

impl Guess {
    pub fn new(val: u32) -> Self {
        if val < 1 || val > 100 {
            panic!("Value should be between 1 and 100");
        }

        Guess {
            value: val
        }
    }

    pub fn value(&self) -> u32 {
        return self.value;
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nPlease enter you guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input! Please enter a number");
                continue;
            }
        };

        let guess = Guess::new(guess);
        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
