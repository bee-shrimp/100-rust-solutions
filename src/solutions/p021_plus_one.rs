//! # Plus One
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Add one to a number represented as array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/021_plus_one.md

/// Solution for Plus One
pub fn solve(int: Vec<i32>) -> Vec<i32> {
    // TODO: Implement your solution here

    // [1,2,3] -> [1,2,4]

    if int.is_empty() {
        return int;
    }

    let mut result = int.clone();
    let len = result.len();
    let mut ptr = 0;

    while let Some(i) = result.iter_mut().nth_back(ptr) {
        if *i == 9 && ptr == len - 1 {
            *i = 0;
            result.insert(0, 1);
            break;
        } else if *i == 9 && ptr < len - 1 {
            *i = 0;
            ptr += 1
        } else {
            *i += 1;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(solve(vec![1, 2, 3]), vec![1, 2, 4])
    }
    #[test]
    fn test_999() {
        assert_eq!(solve(vec![9, 9, 9]), vec![1, 0, 0, 0])
    }
}
