//! # Leap Year
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Determine if a year is a leap year
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/013_leap_year.md

/// Solution for Leap Year
pub fn solve(int: i32) -> bool {
    // TODO: Implement your solution here

    (int % 4 == 0 && int % 100 != 0) || int % 400 == 0
}

// NOTE: could have been shorter.

// og:
// pub fn solve(int: i32) -> bool {
//     // TODO: Implement your solution here
//
//     if int % 4 != 0 {
//         return false;
//     }
//
//     if int % 400 == 0 {
//         return true;
//     }
//
//     if int % 100 == 0 {
//         return false;
//     }
//
//     true
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2000() {
        assert!(solve(2000))
    }

    #[test]
    fn test_1900() {
        assert!(!solve(1900))
    }

    #[test]
    fn test_2024() {
        assert!(solve(2024))
    }
}
