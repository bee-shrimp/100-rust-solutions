//! # Sqrt(x)
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Calculate square root of x
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/022_sqrt_x.md

/// Solution for Sqrt(x)
pub fn solve(x: i32) -> i32 {
    // TODO: Implement your solution here

    let mut high = i64::from(x);
    let mut low = 0;
    let mut mid: i64;

    while low <= high {
        mid = (low + high) / 2;
        if mid * mid <= x as i64 {
            low = mid + 1;
        } else if mid * mid > x as i64 {
            high = mid - 1;
        }
    }
    high as i32
}

// og attempt(doesn't work with 8)
// pub fn solve(x: i32) -> i32 {
//     // TODO: Implement your solution here
//
//     let mut mid: f64 = f64::from(x) / 2.0;
//     let mut prev = mid;
//
//     loop {
//         if mid * mid == x.into() {
//             break mid as i32;
//         } else if mid * mid < x.into() {
//             let new = ((mid + prev) / 2.0).round();
//             prev = mid;
//             mid = new;
//         } else {
//             prev = mid;
//             mid = (mid / 2.0).round();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(solve(25), 5)
    }
    #[test]
    fn test_49() {
        assert_eq!(solve(49), 7)
    }

    #[test]
    fn test_8() {
        assert_eq!(solve(8), 2)
    }
    #[test]
    fn test_big() {
        assert_eq!(solve(2147483647), 46340)
    }
}
