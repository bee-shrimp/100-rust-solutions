//! # Celsius to Fahrenheit
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Convert temperature
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/014_celsius_to_fahrenheit.md

// The formula is: `F = C × 9/5 + 32`

/// Solution for Celsius to Fahrenheit
pub fn solve(cel: f32) -> f32 {
    // TODO: Implement your solution here

    ((cel * 9.0 / 5.0 + 32.0) * 100.0).round() / 100.0
}

// NOTE: rounding to 2 decimal places by ( x * 100).round() / 100.0 is so neat

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(solve(0.0), 32.0)
    }

    #[test]
    fn test_100() {
        assert_eq!(solve(100.0), 212.0)
    }

    #[test]
    fn test_37() {
        assert_eq!(solve(37.0), 98.6)
    }
}
