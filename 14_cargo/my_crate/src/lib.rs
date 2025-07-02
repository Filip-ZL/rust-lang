//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one number to another
/// 
/// # Examples
/// 
/// ```
/// let x = 3;
/// let y = 3;
/// let answer = my_crate::add(x, y);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
