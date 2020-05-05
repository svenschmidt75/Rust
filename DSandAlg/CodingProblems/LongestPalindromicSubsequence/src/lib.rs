// Data Structures & Algorithms ! - https://www.udemy.com/course/draft/1330262/learn/lecture/13947016#overview
// S12.4 - L10 -- Longest Palindromic Subsequence

use std::cmp;

fn lps(s: &str) -> u32 {
    // SS: Divide & Conquer approach
    lps_dq(s, "", 0)
}

fn is_palindrome(s: &str) -> bool {
    let mid = s.len() / 2;
    for i in 0..mid {
        let c1 = s.chars().nth(i).unwrap();
        let c2 = s.chars().nth(s.len() - 1 - i).unwrap();
        if c1 != c2 {
            return false;
        }
    }
    true
}

fn lps_dq(s: &str, prefix: &str, index: usize) -> u32 {
    // SS: base case
    if index == s.len() {
        // SS: check if prefix is a palindrome
        if is_palindrome(prefix) {
            prefix.len() as u32
        } else {
            0
        }
    } else {
        let c = s.chars().nth(index).unwrap();

        // SS: include character
        let mut new_prefix = prefix.to_owned();
        new_prefix.push(c);
        let c1 = lps_dq(s, &new_prefix, index + 1);

        // SS: exclude character
        let c2 = lps_dq(s, prefix, index + 1);

        cmp::max(c1, c2)
    }
}

fn lps2(s: &str, idx1: usize, idx2: usize) -> u32 {
    if idx1 > idx2 {
        // SS: not a palindrome
        0
    } else if idx1 == idx2 {
        // SS: is a palindrome
        1
    } else if s.chars().nth(idx1).unwrap() == s.chars().nth(idx2).unwrap() {
        // SS: characters are the same
        2 + lps2(s, idx1 + 1, idx2 - 1)
    } else {
        let c1 = lps2(s, idx1 + 1, idx2);
        let c2 = lps2(s, idx1, idx2 - 1);
        cmp::max(c1, c2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lps, lps2};

    #[test]
    fn test11() {
        // Arrange
        let s = "ELRMENMET";

        // Act
        let length = lps(s);

        // Arrange
        assert_eq!(length, 5);
    }

    #[test]
    fn test21() {
        // Arrange
        let s = "ELRMENMET";

        // Act
        let length = lps2(s, 0, s.len() - 1);

        // Arrange
        assert_eq!(length, 5);
    }
}
