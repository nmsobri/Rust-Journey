//! # My Crate
//! This is to explain what is My Crate is
//! It is actually nothing lol

/// Add one to the given number
///
/// # Examples
///
/// ```
/// let x = 4;
/// assert_eq!(5, my_crate::add_one(x));
/// ```
///
/// # Errors
/// It will return `Result`
/// The variant is
///
pub fn add_one(x: u32) -> u32 {
    x + 1
}