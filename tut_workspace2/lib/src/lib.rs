pub mod util {
    pub fn lorem() {
        println!("Lorem");
    }

    pub fn ipsum() {
        println!("Ipsum");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
