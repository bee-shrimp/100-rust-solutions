//! # Prime Number Check
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Check if a number is prime
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/012_prime_number_check.md

/// Solution for Prime Number Check
pub fn solve(int: i32) -> bool {
    // TODO: Implement your solution here

    if int < 2 {
        return false;
    }

    if int == 2 {
        return true;
    }

    if int & 1 == 0 {
        return false;
    }

    let mut i = 3;

    while i <= int.isqrt() {
        if int % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

// NOTE: no need for checking divisibility of 2 since even numbers already return false.

// og:
// pub fn solve(int: i32) -> bool {
//     // TODO: Implement your solution here
//
//     if int < 2 {
//         return false;
//     }
//
//     if int == 2 {
//         return true;
//     }
//
//     if int & 1 == 0 {
//         return false;
//     }
//
//     for i in 2..=int.isqrt() {
//         if int % i == 0 {
//             return false;
//         }
//     }
//
//     true
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert!(!solve(9))
    }

    #[test]
    fn test_zero() {
        assert!(!solve(0))
    }

    #[test]
    fn test_one() {
        assert!(!solve(1))
    }

    #[test]
    fn test_two() {
        assert!(solve(2))
    }

    #[test]
    fn test_big() {
        assert!(solve(97))
    }
}
