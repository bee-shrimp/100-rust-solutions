//! # Even or Odd
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Determine if a number is even or odd
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/011_even_or_odd.md

/// Solution for Even or Odd
pub fn solve(int: i32) -> String {
    // TODO: Implement your solution here

    match int & 1 {
        0 => String::from("Even"),
        _ => String::from("Odd"),
    }
}

// og:
// pub fn solve(int: i32) -> String {
//     // TODO: Implement your solution here
//
//     match int % 2 {
//         0 => String::from("Even"),
//         _ => String::from("Odd"),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even() {
        assert_eq!(solve(4), "Even")
    }

    #[test]
    fn test_odd() {
        assert_eq!(solve(5), "Odd")
    }

    #[test]
    fn test_zero() {
        assert_eq!(solve(0), "Even")
    }
}
