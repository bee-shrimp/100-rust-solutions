//! # Count Vowels
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Count vowels in a string
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/017_count_vowels.md

/// Solution for Count Vowels
pub fn solve(str: &str) -> usize {
    // TODO: Implement your solution here

    str.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(solve("apple"), 2)
    }

    #[test]
    fn test_capital() {
        assert_eq!(solve("AEIOU"), 5)
    }
}
