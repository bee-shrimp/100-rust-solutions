//! # Swap Two Numbers
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Swap two numbers without temp
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/015_swap_two_numbers.md

/// Solution for Swap Two Numbers
pub fn solve(a: &mut i32, b: &mut i32) -> (i32, i32) {
    // TODO: Implement your solution here
    std::mem::swap(a, b);
    (*a, *b)
}

// NOTE: misunderstood the problem?

// og:
// pub fn solve(a: i32, b: i32) -> (i32, i32) {
//     // TODO: Implement your solution here

//     (b, a)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(solve(&mut 5, &mut 10), (10, 5))
    }
}
