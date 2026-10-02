//! # Valid Parentheses
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Check if parentheses are valid
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/020_valid_parentheses.md

/// Solution for Valid Parentheses
pub fn solve(s: String) -> bool {
    // TODO: Implement your solution here
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' => {
                if !stack.pop().is_some_and(|stack_item| stack_item == '(') {
                    return false;
                }
            }
            ']' => {
                if !stack.pop().is_some_and(|stack_item| stack_item == '[') {
                    return false;
                }
            }
            '}' => {
                if !stack.pop().is_some_and(|stack_item| stack_item == '{') {
                    return false;
                }
            }

            _ => panic!("there should not be any other char"),
        }
    }
    stack.is_empty()
}

// og:
// pub fn solve(s: String) -> bool {
//     // TODO: Implement your solution here
//     let mut stack: Vec<char> = Vec::new();
//
//     s.chars().all(|c| match c {
//         '(' => {
//             stack.push(c);
//             true
//         }
//         ')' => stack.pop().is_some_and(|stack_item| stack_item == '('),
//         '[' => {
//             stack.push(c);
//             true
//         }
//         ']' => stack.pop().is_some_and(|stack_item| stack_item == '['),
//         '{' => {
//             stack.push(c);
//             true
//         }
//         '}' => stack.pop().is_some_and(|stack_item| stack_item == '{'),
//
//         _ => panic!("there should not be any other char"),
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let s = "([)]".to_string();
        assert!(!solve(s))
    }

    #[test]
    fn test_true() {
        let s = "()[]{}".to_string();
        assert!(solve(s))
    }
}
