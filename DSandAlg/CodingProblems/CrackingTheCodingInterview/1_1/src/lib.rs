// Cracking the Coding Interview
// 6th ed, p. 90, ex. 1.1

use std::collections::{HashMap, HashSet};

fn unique_1(input: &str) -> bool {
    // SS: Use frequency map, for O(n) runtime, O(n) space
    let mut frequency_map = HashSet::new();
    for c in input.chars() {
        if frequency_map.contains(&c) {
            return false;
        }
        frequency_map.insert(c);
    }
    true
}

fn unique_2(input: &str) -> bool {
    // SS: no extra data structure allowed, total runtime O(n log n)
    if input.is_empty() {
        true
    } else {
        // SS: sort at O(n log n)
        let mut input_sorted: Vec<char> = input.chars().collect();
        input_sorted.sort();

        // SS: O(n)
        for i in 0..(input_sorted.len() - 1) {
            if input_sorted[i] == input_sorted[i + 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let input = "abcd";

        // Act
        let result = unique_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test12() {
        // Arrange
        let input = "abcda";

        // Act
        let result = unique_1(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test13() {
        // Arrange
        let input = "";

        // Act
        let result = unique_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test21() {
        // Arrange
        let input = "abcd";

        // Act
        let result = unique_2(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test22() {
        // Arrange
        let input = "abcda";

        // Act
        let result = unique_2(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test23() {
        // Arrange
        let input = "";

        // Act
        let result = unique_2(input);

        // Assert
        assert_eq!(result, true);
    }
}
