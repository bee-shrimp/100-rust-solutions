//! # Find Maximum
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Find the maximum element in an array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/009_find_maximum.md

/// Solution for Find Maximum
pub fn solve(ints: &[i32]) -> i32 {
    // TODO: Implement your solution here

    *ints.iter().max().unwrap()
}

// NOTE: could have used .copied()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(&[1, 5, 3, 9, 2]), 9)
    }
}
