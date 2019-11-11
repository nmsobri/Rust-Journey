//! Art
//! Library for modelling artistic art

pub use kind::PrimaryColor;
pub use kind::SecondaryColor;
pub use util::mix;

pub mod kind {
    /// Primary Color
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary Color
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod util {
    use crate::kind::*;

    pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> SecondaryColor {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
