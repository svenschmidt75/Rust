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
            cmp::max(longest, cmp::max(c1, c2))
        }
    }
}

fn lcs_bottom_up(s1: &str, s2: &str) -> u64 {
    // SS: DP, bottom-up approach, runtime is O(s1 * s2)
    let mut grid = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    for i in 0..s1.len() {
        let i1 = s1.len() - 1 - i;
        let c1 = s1.chars().nth(i1).unwrap();

        for j in 0..s2.len() {
            let i2 = s2.len() - 1 - j;
            let c2 = s2.chars().nth(i2).unwrap();

            if c1 == c2 {
                grid[i2][i1] = 1;

                // SS: if chars are the same at cells (i2+1,i1_1), than add that
                // cell value to the current cell value, otherwise leave at 1
                if i1 < s1.len() - 1 && i2 < s2.len() - 1 {
                    let c1 = s1.chars().nth(i1 + 1).unwrap();
                    let c2 = s2.chars().nth(i2 + 1).unwrap();
                    if c1 == c2 {
                        grid[i2][i1] += grid[i2 + 1][i1 + 1];
                    }
                }
            } else {
                let cell1 = grid[i2 + 1][i1];
                let cell2 = grid[i2][i1 + 1];
                grid[i2][i1] = cmp::max(cell1, cell2);
            }
        }
    }

    grid[0][0]
}

#[cfg(test)]
mod tests {
    use crate::{lcs, lcs_bottom_up};

    #[test]
    fn test1() {
        // Arrange
        let s1 = "hish";
        let s2 = "fish";

        // Act
        let longest1 = lcs(s1, s2, 0, 0, 0);
        let longest2 = lcs_bottom_up(s1, s2);

        // Assert
        assert_eq!(longest1, 3);
        assert_eq!(longest1, longest2);
    }

    #[test]
    fn test2() {
        // Arrange
        let s1 = "hish";
        let s2 = "vista";

        // Act
        let longest1 = lcs(s1, s2, 0, 0, 0);
        let longest2 = lcs_bottom_up(s1, s2);

        // Assert
        assert_eq!(longest1, 2);
        assert_eq!(longest1, longest2);
    }

    #[test]
    fn test3() {
        // Arrange
        let s1 = "GeeksforGeeks";
        let s2 = "GeeksQuiz";

        // Act
        let longest1 = lcs(s1, s2, 0, 0, 0);
        let longest2 = lcs_bottom_up(s1, s2);

        // Assert
        assert_eq!(longest1, 5);
        assert_eq!(longest1, longest2);
    }

    #[test]
    fn test4() {
        // Arrange
        let s1 = "abcdxyz";
        let s2 = "xyzabcd";

        // Act
        let longest1 = lcs(s1, s2, 0, 0, 0);
        let longest2 = lcs_bottom_up(s1, s2);

        // Assert
        assert_eq!(longest1, 4);
        assert_eq!(longest1, longest2);
    }

    #[test]
    fn test5() {
        // Arrange
        let s1 = "zxabcdezy";
        let s2 = "yzabcdezx";

        // Act
        let longest1 = lcs(s1, s2, 0, 0, 0);
        let longest2 = lcs_bottom_up(s1, s2);

        // Assert
        assert_eq!(longest1, 6);
        assert_eq!(longest1, longest2);
    }
}
