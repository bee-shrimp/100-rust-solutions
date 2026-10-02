//! # Merge Sorted Arrays
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Merge two sorted arrays
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/019_merge_two_sorted_arrays.md

// NOTE: answer has different function signeture from problem

use std::result;

/// Solution for Merge Sorted Arrays
pub fn solve(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // TODO: Implement your solution here

    let mut result = [nums1, nums2].concat();
    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 5, 6];
        let expected = vec![1, 2, 2, 3, 5, 6];
        let result = solve(nums1, nums2);
        assert_eq!(result, expected)
    }
}
