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

fn solve(input: &str) -> u32 {
    // SS: runtime complexity: O(N), space: O(1)
    // assuming all chars are lower-case
    let mut max_length = 0;
    let mut length = 0;
    let mut pos = 0;

    let cs = input.chars().collect::<Vec<_>>();

    let mut characters: u32 = 0;

    while pos < input.len() {
        let c = cs[pos];
        if find_character(c, characters) {
            max_length = cmp::max(max_length, length);
            length = 1;
            characters = 0;
        } else {
            length += 1;
        }
        set_character(c, &mut characters);
        pos += 1;
    }

    max_length = cmp::max(max_length, length);
    max_length
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        // Arrange
        let input = "abcabcbb";

        // Act
        let max_length = solve(input);

        // Assert
        assert_eq!(max_length, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = "bbbbb";

        // Act
        let max_length = solve(input);

        // Assert
        assert_eq!(max_length, 1);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = "pwwkew";

        // Act
        let max_length = solve(input);

        // Assert
        assert_eq!(max_length, 3);
    }
}
