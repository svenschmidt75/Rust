// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::cmp;

fn find_character(c: char, characters: u32) -> bool {
    let n = c as u8 - 'a' as u8;
    let bit = 1 << n;
    let bit_set = characters & bit;
    bit_set > 0
}

fn set_character(c: char, characters: &mut u32) {
    let n = c as u8 - 'a' as u8;
    let bit = 1 << n;
    *characters = *characters | bit;
}

fn clear_character(c: char, characters: &mut u32) {
    let n = c as u8 - 'a' as u8;
    let bit = 1 << n;
    *characters = *characters & (!bit);
}

fn solve_slow(input: &str) -> u32 {
    // SS: runtime complexity: O(N^2), space: O(1)
    // assuming all chars are lower-case
    let mut max_length = 1;
    let mut length = 0;
    let mut pos = 0;

    let cs = input.chars().collect::<Vec<_>>();

    let mut characters: u32 = 0;

    while pos < input.len() {
        characters = 0;
        let c = cs[pos];
        set_character(c, &mut characters);
        length = 1;
        pos += 1;

        let mut other_pos = pos;
        while other_pos < input.len() {
            let c = cs[other_pos];
            if find_character(c, characters) {
                break;
            } else {
                length += 1;
            }
            set_character(c, &mut characters);
            other_pos += 1;
        }
        max_length = cmp::max(max_length, length);
    }

    max_length = cmp::max(max_length, length);
    max_length
}

fn solve_fast(input: &str) -> u32 {
    // SS: use sliding window, runtime complexity: O(N), space: O(1)
    // assuming all chars are lower-case
    let mut max_length = 0;

    let cs = input.chars().collect::<Vec<_>>();

    let mut characters: u32 = 0;

    let mut i = 0;
    let mut j = 0;

    while i < input.len() && j < input.len() {
        let c = cs[j];
        if find_character(c, characters) {
            let c = cs[i];
            clear_character(c, &mut characters);
            i += 1;
        } else {
            set_character(c, &mut characters);
            j += 1;
            max_length = cmp::max(max_length, j - i);
        }
    }
    max_length as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let input = "abcabcbb";

        // Act
        let max_length = solve_slow(input);

        // Assert
        assert_eq!(max_length, 3);
    }

    #[test]
    fn test12() {
        // Arrange
        let input = "abcabcbb";

        // Act
        let max_length = solve_fast(input);

        // Assert
        assert_eq!(max_length, 3);
    }

    #[test]
    fn test21() {
        // Arrange
        let input = "bbbbb";

        // Act
        let max_length = solve_slow(input);

        // Assert
        assert_eq!(max_length, 1);
    }

    #[test]
    fn test22() {
        // Arrange
        let input = "bbbbb";

        // Act
        let max_length = solve_fast(input);

        // Assert
        assert_eq!(max_length, 1);
    }

    #[test]
    fn test31() {
        // Arrange
        let input = "pwwkew";

        // Act
        let max_length = solve_slow(input);

        // Assert
        assert_eq!(max_length, 3);
    }

    #[test]
    fn test32() {
        // Arrange
        let input = "pwwkew";

        // Act
        let max_length = solve_fast(input);

        // Assert
        assert_eq!(max_length, 3);
    }

    #[test]
    fn test41() {
        // Arrange
        let input = "abcade";

        // Act
        let max_length = solve_slow(input);

        // Assert
        assert_eq!(max_length, 5);
    }

    #[test]
    fn test42() {
        // Arrange
        let input = "abcade";

        // Act
        let max_length = solve_fast(input);

        // Assert
        assert_eq!(max_length, 5);
    }
}
