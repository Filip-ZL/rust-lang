// try to run: cargo test -- --show-output
pub fn add(left: u64, right: u64) -> u64 {
    println!("Adding {} + {} = {}", left, right, left + right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_five_and_four() {
        let result = add(5, 4);
        assert_eq!(result, 9);
    }

    #[test]
    fn one_houndred() {
        let result = add(50, 50);
        assert_eq!(result, 100)
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
