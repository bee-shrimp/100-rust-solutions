//! # Sum of Array
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Calculate sum of all array elements
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/008_sum_of_array.md

/// Solution for Sum of Array
pub fn solve(ints: &[i32]) -> i32 {
    // TODO: Implement your solution here

    ints.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(&[1, 2, 3, 4, 5]), 15)
    }
}
