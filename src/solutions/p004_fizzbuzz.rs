//! # FizzBuzz
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Classic FizzBuzz problem
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/004_fizzbuzz.md

/// Solution for FizzBuzz
pub fn solve(int: i32) -> Vec<String> {
    // TODO: Implement your solution here

    (1..=int)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        })
        .collect()
}

// og:
// pub fn solve(int: i32) -> Vec<String> {
//     // TODO: Implement your solution here
//
//     let mut answer: Vec<String> = Vec::new();
//
//     for i in 1..=int {
//         if i % 3 == 0 && i % 5 == 0 {
//             answer.push("FizzBuzz".to_string());
//         } else if i % 3 == 0 {
//             answer.push("Fizz".to_string());
//         } else if i % 5 == 0 {
//             answer.push("Buzz".to_string());
//         }
//         answer.push(i.to_string());
//     }
//     answer
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let int = 15;
        let expected = [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(solve(int), expected);
        // TODO: Add test cases
    }
}
