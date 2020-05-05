// Data Structures & Algorithms ! - https://www.udemy.com/course/draft/1330262/learn/lecture/13950508#overview
// S12.4 - L11 -- Longest Palindromic Substring

use std::cmp;

fn lps(s: &str, idx1: usize, idx2: usize) -> u32 {
    if idx1 > idx2 {
        0
    } else if idx1 == idx2 {
        1
    } else {
        let mut c1 = 0;
        if s.chars().nth(idx1).unwrap() == s.chars().nth(idx2).unwrap() {
            let remaining_length = (idx2 - idx1 - 1) as u32;
            if remaining_length == lps(s, idx1 + 1, idx2 - 1) {
                c1 = 2 + remaining_length;
            }
        }
        let c2 = lps(s, idx1 + 1, idx2);
        let c3 = lps(s, idx1, idx2 - 1);
        cmp::max(c1, cmp::max(c2, c3))
    }
}

#[cfg(test)]
mod tests {
    use crate::lps;

    #[test]
    fn test1() {
        // Arrange
        let s = "ABCYRCFBTUA";

        // Act
        let longest = lps(s, 0, s.len() - 1);

        // Assert
        assert_eq!(longest, 1);
    }

    #[test]
    fn test2() {
        // Arrange
        let s = "ABCCBUA";

        // Act
        let longest = lps(s, 0, s.len() - 1);

        // Assert
        assert_eq!(longest, 4);
    }
}
