// Grokking Algorithms, Manning, Aditya B. Bhargava
// p. Chapter 9, longest common substring
// https://www.geeksforgeeks.org/longest-common-substring-dp-29/

use std::cmp;

fn lcs(s1: &str, s2: &str, i1: usize, i2: usize, longest: u64) -> u64 {
    // SS: Divide and Conquer, runtime is O(2^n), n=min(s1.length, s2.length)

    if i1 == s1.len() || i2 == s2.len() {
        longest
    } else {
        let c1 = s1.chars().nth(i1).unwrap();
        let c2 = s2.chars().nth(i2).unwrap();

        if c1 == c2 {
            lcs(s1, s2, i1 + 1, i2 + 1, longest + 1)
        } else {
            // SS: 2 options

            // skip char in s1
            let c1 = lcs(s1, s2, i1 + 1, i2, 0);

            // skip char in s2
            let c2 = lcs(s1, s2, i1, i2 + 1, 0);

            // maximum of so far (longest) and all 2 options
            cmp::max(longest, cmp::max(c1, c2, ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lcs;

    #[test]
    fn test1() {
        // Arrange
        let s1 = "hish";
        let s2 = "fish";

        // Act
        let longest = lcs(s1, s2, 0, 0, 0);

        // Assert
        assert_eq!(longest, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let s1 = "hish";
        let s2 = "vista";

        // Act
        let longest = lcs(s1, s2, 0, 0, 0);

        // Assert
        assert_eq!(longest, 2);
    }

    #[test]
    fn test3() {
        // Arrange
        let s1 = "GeeksforGeeks";
        let s2 = "GeeksQuiz";

        // Act
        let longest = lcs(s1, s2, 0, 0, 0);

        // Assert
        assert_eq!(longest, 5);
    }

    #[test]
    fn test4() {
        // Arrange
        let s1 = "abcdxyz";
        let s2 = "xyzabcd";

        // Act
        let longest = lcs(s1, s2, 0, 0, 0);

        // Assert
        assert_eq!(longest, 4);
    }

    #[test]
    fn test5() {
        // Arrange
        let s1 = "zxabcdezy";
        let s2 = "yzabcdezx";

        // Act
        let longest = lcs(s1, s2, 0, 0, 0);

        // Assert
        assert_eq!(longest, 6);
    }
}
