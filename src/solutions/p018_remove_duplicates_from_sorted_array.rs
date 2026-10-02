//! # Remove Duplicates
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Remove duplicates from sorted array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/018_remove_duplicates_from_sorted_array.md

/// Solution for Remove Duplicates
pub fn solve(vec: &mut Vec<i32>) -> i32 {
    // TODO: Implement your solution here

    if vec.is_empty() {
        return 0;
    }

    let mut unique = 0;
    let mut count = 1;

    for i in 1..vec.len() {
        if vec[i] != vec[unique] {
            vec[unique + 1] = vec[i];
            unique = count;
            count += 1;
        }
    }

    vec.truncate(count);

    count as i32
}

// NOTE: misunderstood problem

// og:
// use std::collections::HashSet;
// pub fn solve(vec: &[i32]) -> i32 {
//     // TODO: Implement your solution here

//     let mut set = HashSet::new();
//
//     vec.iter().for_each(|x| {
//         if !set.contains(&x) {
//             set.insert(x);
//         }
//     });
//
//     set.len() as i32
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = solve(&mut vec);
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
        assert_eq!(result, 5);
    }
}
