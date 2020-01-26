// https://leetcode.com/problems/valid-parentheses/

use std::collections::VecDeque;

fn has_valid_parentheses(input: &str) -> bool {
    let mut stack = VecDeque::new();
    for c in input.chars() {
        let res = match c {
            '(' => {
                stack.push_back(')');
                true
            }
            '[' => {
                stack.push_back(']');
                true
            }
            '{' => {
                stack.push_back('}');
                true
            }
            ')' | ']' | '}' => stack.pop_back().map_or(false, |x| x == c),
            _ => panic!("Unexpected char"),
        };

        if res == false {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::has_valid_parentheses;

    #[test]
    fn test1() {
        // Arrange
        let input = "()";

        // Act
        let is_valid = has_valid_parentheses(input);

        // Arrange
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = "()[]{}";

        // Act
        let is_valid = has_valid_parentheses(input);

        // Arrange
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = "(]";

        // Act
        let is_valid = has_valid_parentheses(input);

        // Arrange
        assert_eq!(is_valid, false);
    }

    #[test]
    fn test4() {
        // Arrange
        let input = "([)]";

        // Act
        let is_valid = has_valid_parentheses(input);

        // Arrange
        assert_eq!(is_valid, false);
    }

    #[test]
    fn test5() {
        // Arrange
        let input = "{[]}";

        // Act
        let is_valid = has_valid_parentheses(input);

        // Arrange
        assert_eq!(is_valid, true);
    }
}
