use rand;
use rand::Rng;

pub fn add_one(x: u32) -> u32 {
    let rn = rand::thread_rng().gen_range(1, 9);
    println!("Random is {}", rn);
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
