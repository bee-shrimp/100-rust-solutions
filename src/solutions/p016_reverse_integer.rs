//! # Reverse Integer
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Reverse digits of an integer
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/016_reverse_integer.md

/// Solution for Reverse Integer
pub fn solve(mut int: i32) -> i32 {
    // TODO: Implement your solution here

    let mut rev: i32 = 0;

    while int.abs() >= 1 {
        let last = int % 10;

        rev = if let Some(mul) = rev.checked_mul(10) {
            mul.checked_add(last).unwrap_or(0)
        } else {
            0
        };

        int /= 10;
    }

    rev
}

// NOTE: learned about checked_mul/add

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(solve(123), 321)
    }

    #[test]
    fn test_negative() {
        assert_eq!(solve(-123), -321)
    }

    #[test]
    fn test_overflow() {
        assert_eq!(solve(1534236469), 0)
    }
}
