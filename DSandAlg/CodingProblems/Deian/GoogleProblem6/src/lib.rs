use std::collections::HashMap;

fn can_be_made_into_palindrome_1(input: &str) -> bool {
    let mut frequency_map = HashMap::new();

    for c in input.chars() {
        let counter = frequency_map.entry(c).or_insert(0);
        *counter += 1;
    }

    let is_even = input.len() % 2 == 0;

    // SS: check that every character occurs an even time, except for an
    // odd-length input, where we allow 1 odd char.
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

fn can_be_made_into_palindrome_2(input: &str) -> bool {
    // SS: we assume the string consists of ASCII chars
    // an is lower-case
    let mut buffer: [u8; 26] = [0; 26];

    for c in input.chars() {
        let index = (c as u8 - 'a' as u8) as usize;
        buffer[index] += 1;
    }

    let is_even = input.len() % 2 == 0;

    let mut odd_count = 0;
    for frequency in buffer.iter() {
        if *frequency % 2 == 1 {
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

fn is_bit_set(value: u32, bit: usize) -> bool {
    value & (1 << bit) as u32 > 0
}

fn set_bit(value: &mut u32, bit: usize) {
    *value |= (1 << bit) as u32
}

fn clear_bit(value: &mut u32, bit: usize) {
    *value &= !(1 << bit) as u32
}

fn flip_bit(input: &mut u32, bit: usize) {
    if is_bit_set(*input, bit) {
        clear_bit(input, bit);
    } else {
        set_bit(input, bit);
    }
}

fn is_power_of_two(input: u32) -> bool {
    input & (input - 1) == 0
}

fn can_be_made_into_palindrome_3(input: &str) -> bool {
    // SS: we assume the string consists of ASCII chars
    // an is lower-case
    let mut buffer: u32 = 0;

    for c in input.chars() {
        let bit = (c as u32 - 'a' as u32) as usize;
        flip_bit(&mut buffer, bit);
    }

    let is_even = input.len() % 2 == 0;
    if is_even {
        return buffer == 0;
    }

    // SS: if the string in odd-length, only one bit must be set,
    // so check for power of two...
    is_power_of_two(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even1() {
        // Arrange
        let input = "abba";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_odd1() {
        // Arrange
        let input = "madam";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_false1() {
        // Arrange
        let input = "madama";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_odd_false1() {
        // Arrange
        let input = "madai";

        // Act
        let result = can_be_made_into_palindrome_1(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_even2() {
        // Arrange
        let input = "abba";

        // Act
        let result = can_be_made_into_palindrome_2(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_odd2() {
        // Arrange
        let input = "madam";

        // Act
        let result = can_be_made_into_palindrome_2(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_false2() {
        // Arrange
        let input = "madama";

        // Act
        let result = can_be_made_into_palindrome_2(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_odd_false2() {
        // Arrange
        let input = "madai";

        // Act
        let result = can_be_made_into_palindrome_2(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_even3() {
        // Arrange
        let input = "abba";

        // Act
        let result = can_be_made_into_palindrome_3(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_odd3() {
        // Arrange
        let input = "madam";

        // Act
        let result = can_be_made_into_palindrome_3(input);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_false3() {
        // Arrange
        let input = "madama";

        // Act
        let result = can_be_made_into_palindrome_3(input);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test_odd_false3() {
        // Arrange
        let input = "madai";

        // Act
        let result = can_be_made_into_palindrome_3(input);

        // Assert
        assert_eq!(result, false);
    }
}
