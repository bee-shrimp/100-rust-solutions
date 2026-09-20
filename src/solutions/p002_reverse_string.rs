//! # Reverse String
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Reverse a string in-place
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/002_reverse_string.md

/// Solution for Reverse String
pub fn rev(s: &mut [char]) {
    // TODO: Implement your solution here

    let mut left = 0;
    let mut right = s.len().saturating_sub(1);

    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

// og:
// pub fn rev(s: &mut Vec<char>) {
//     // TODO: Implement your solution here
//
//     for i in 0..s.len() {
//         let last = s.pop().unwrap();
//         s.insert(i, last);
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        rev(&mut s);
        assert_eq!(s, expected);
    }
}
