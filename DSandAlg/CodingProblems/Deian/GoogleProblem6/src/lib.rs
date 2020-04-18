use std::collections::HashMap;

fn can_be_made_into_palindrome_1(input: &str) -> bool {
    let mut frequency_map = HashMap::new();
    for c in input.chars() {
        let counter = frequency_map.entry(c).or_insert(0);
        *counter += 1;
    }

    let is_even = input.len() % 2 == 0;

    let mut odd_count = 0;
    for (_, count) in frequency_map.iter() {
        if *count % 2 == 1 {
            if is_even {
                return false;
            } else if odd_count == 1 {
                return false;
            }
            odd_count += 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::can_be_made_into_palindrome_1;

    #[test]
    fn test_even() {
        // Arrange
        let input = "abba";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_odd() {
        // Arrange
        let input = "madam";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_false() {
        // Arrange
        let input = "madama";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_odd_false() {
        // Arrange
        let input = "madai";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, false);
    }
}
