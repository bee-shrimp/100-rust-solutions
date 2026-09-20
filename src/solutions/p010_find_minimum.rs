//! # Find Minimum
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Find the minimum element in an array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/010_find_minimum.md

/// Solution for Find Minimum
pub fn solve(ints: &[i32]) -> i32 {
    // TODO: Implement your solution here

    ints.iter().copied().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(&[1, 5, 3, 9, 2]), 1)
    }
}
