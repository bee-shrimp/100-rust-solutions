//! # Palindrome Number
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Check if a number is a palindrome
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/003_palindrome_number.md

/// Solution for Palindrome Number
pub fn solve(int: i32) -> bool {
    // TODO: Implement your solution here

    if int.is_negative() || (int % 10 == 0 && int != 0) {
        return false;
    }

    let mut num = int;
    let mut rev = 0;

    while num > rev {
        let last = num % 10;
        rev = rev * 10 + last;
        num /= 10;
    }

    num == rev || num == rev / 10
}

// og:
// pub fn solve(int: i32) -> bool {
//     // TODO: Implement your solution here
//
//     let int = int.to_string();
//     int.chars()
//         .enumerate()
//         .all(|(i, num)| num == int.chars().nth_back(i).unwrap())
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_true() {
        // TODO: Add test cases
        assert!(solve(12321));
    }

    #[test]
    fn test_false() {
        // TODO: Add test cases
        assert!(!solve(-121));
    }
}
