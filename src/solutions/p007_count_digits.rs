//! # Count Digits
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Count the number of digits in a number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/007_count_digits.md

/// Solution for Count Digits
pub fn solve(mut int: i32) -> i32 {
    // TODO: Implement your solution here

    let mut answer = 1;
    while 10 <= int {
        int /= 10;
        answer += 1
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(12345), 5)
    }
}
