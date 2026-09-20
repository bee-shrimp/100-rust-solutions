//! # Two Sum
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Find indices of two numbers that add up to target
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/001_two_sum.md

use std::collections::HashMap;

/// Solution for Two Sum
pub fn solve(nums: &[i32], target: i32) -> [usize; 2] {
    // TODO: Implement your solution here

    let mut dict: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(idx) = dict.get(&(target - num)) {
            return [i, *idx];
        }
        dict.insert(*num, i);
    }

    panic!("no solution found");
}

// og2:
// pub fn solve(nums: &[i32], target: i32) -> [i32; 2] {
//     // TODO: Implement your solution here
//
//     let mut found = [0, 0];
//     for i in 0..nums.len() {
//         let to_find = target - nums[i];
//
//         if let Some(j) = nums.iter().skip(i).position(|j| *j == to_find) {
//             found = [i32::try_from(i).unwrap(), i32::try_from(j).unwrap()];
//         }
//     }
//     found
// }

// og1:
// pub fn solve(nums: &[i32], target: i32) -> [i32; 2] {
//     // TODO: Implement your solution here
//
//     let mut res = [0, 0];
//
//     for i in 0..nums.len() {
//         for j in i + 1..nums.len() {
//             if nums[i] + nums[j] == target {
//                 res = [i32::try_from(i).unwrap(), i32::try_from(j).unwrap()];
//             }
//         }
//     }
//     res
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let nums = [3, 3];
        let target = 6;
        assert_eq!(solve(&nums, target), [1, 0]);

        // TODO: Add test cases
    }
}
