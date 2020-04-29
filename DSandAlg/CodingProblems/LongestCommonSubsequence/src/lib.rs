// Leetcode 1143, https://leetcode.com/problems/longest-common-subsequence/
// and Udemy, Data Structures & Algorithms !, L12.4 - L09

use std::cmp;

fn longest_common_subsequence(s1: &str, s2: &str, i1: usize, i2: usize) -> u64 {
    // SS: Divide & Conquor, runtime is O(2^n), where n=min(s1.length, s2.length)

    if i1 >= s1.len() || i2 >= s2.len() {
        0
    } else {
        let a1 = s1.chars().nth(i1).unwrap();
        let a2 = s2.chars().nth(i2).unwrap();

        if a1 == a2 {
            1 + longest_common_subsequence(s1, s2, i1 + 1, i2 + 1)
        } else {
            // SS: 2 options

            // 1. remove char from s1
            let c1 = longest_common_subsequence(s1, s2, i1 + 1, i2);

            // 2. remove char from s2
            let c2 = longest_common_subsequence(s1, s2, i1, i2 + 1);

            cmp::max(c1, c2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_common_subsequence;

    #[test]
    fn test1() {
        // Assert
        let s1 = "elephant";
        let s2 = "eretpat";

        // Act
        let longest_cnt = longest_common_subsequence(s1, s2, 0, 0);

        // Assert
        assert_eq!(longest_cnt, 5);
    }

    #[test]
    fn test2() {
        // Assert
        let s1 = "abcde";
        let s2 = "ace";

        // Act
        let longest_cnt = longest_common_subsequence(s1, s2, 0, 0);

        // Assert
        assert_eq!(longest_cnt, 3);
    }

    #[test]
    fn test3() {
        // Assert
        let s1 = "abc";
        let s2 = "abc";

        // Act
        let longest_cnt = longest_common_subsequence(s1, s2, 0, 0);

        // Assert
        assert_eq!(longest_cnt, 3);
    }

    #[test]
    fn test4() {
        // Assert
        let s1 = "abc";
        let s2 = "def";

        // Act
        let longest_cnt = longest_common_subsequence(s1, s2, 0, 0);

        // Assert
        assert_eq!(longest_cnt, 0);
    }
}
