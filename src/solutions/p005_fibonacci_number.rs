//! # Fibonacci Number
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Calculate the nth Fibonacci number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/005_fibonacci_number.md

/// Solution for Fibonacci Number
pub fn solve(int: i32) -> i32 {
    // TODO: Implement your solution here

    if int < 2 {
        return int;
    }

    let mut f1 = 0;
    let mut f2 = 1;

    for _ in 2..=int {
        let fib = f1 + f2;
        f1 = f2;
        f2 = fib;
    }

    f2
}

// NOTE: it seems like im always creating unnecessary vecs.
// be more comfortable with using temporary variables.

// og:
// pub fn solve(int: i32) -> i32 {
//     // TODO: Implement your solution here
//
//     let mut fibs: Vec<i32> = Vec::with_capacity(int as usize);
//
//     for i in 0..=int {
//         if fibs.len() < 2 {
//             fibs.push(i);
//             continue;
//         }
//
//         let f1 = fibs[i as usize - 2];
//         let f2 = fibs[i as usize - 1];
//         let fib = f1 + f2;
//
//         fibs.push(fib);
//     }
//
//     fibs[int as usize]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(solve(10), 55);
    }
}
