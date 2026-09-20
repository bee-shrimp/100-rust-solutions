//! # Factorial
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Calculate factorial of a number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/006_factorial.md

/// Solution for Factorial
pub fn solve(int: i32) -> i64 {
    // TODO: Implement your solution here

    (1..=int as i64).product()
}

// NOTE: i64 because 13! already overflows i32.

// og:
// pub fn solve(int: i32) -> i32 {
//     // TODO: Implement your solution here
//
//     if int == 0 {
//         return 1;
//     }
//
//     let mut answer = 1;
//     for i in 1..=int {
//         answer *= i
//     }
//
//     answer
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(5), 120)
    }
}
