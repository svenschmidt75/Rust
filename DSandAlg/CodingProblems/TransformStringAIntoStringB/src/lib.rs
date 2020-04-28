// Data Structures & Algorithms !
// Convert one string into another using DP
// S12.4 - L07

use std::cmp;

fn transform(s1: &str, s2: &str, i1: usize, i2: usize, buffer: &mut [Vec<usize>]) -> usize {
    // SS: This Divide & Conquor solution has runtime complexity O(3^{N})!
    // DP adds memoization to this, in order to reduce it to linear time O(N)!
    // via buffer
    if i1 == s1.len() && i2 == s2.len() {
        // SS: both strings exhausted, nothing to do
        0
    } else if i1 == s1.len() {
        // SS: we have to remove the remaining characters
        s2.len() - i2
    } else if i2 == s2.len() {
        // SS: we have to add the remaining characters
        s1.len() - i1
    } else {
        // SS: both strings have the same length, so 3 options:
        let c1 = s1.chars().nth(i1).unwrap();
        let c2 = s2.chars().nth(i2).unwrap();

        // SS: if both chars are the same, nothing to do
        if c1 == c2 {
            transform(s1, s2, i1 + 1, i2 + 1, buffer)
        } else {
            // SS: use cache?
            if buffer[i1][i2] == 0 {
                // SS: 3 options:

                // 1. we replace the char in s2
                // Example:
                // s2 = a
                // s1 = b
                // i1 = i2 = 0
                let a1 = 1 + transform(s1, s2, i1 + 1, i2 + 1, buffer);

                // 2. we remove the char in s2
                // Example:
                // s2 = ab
                // s1 =  b
                // i1 = i2 = 0
                let a2 = 1 + transform(s1, s2, i1, i2 + 1, buffer);

                // 3. we insert the char into s2
                // Example:
                // s2 = b
                // s1 = ab
                // i1 = i2 = 0
                let a3 = 1 + transform(s1, s2, i1 + 1, i2, buffer);

                buffer[i1][i2] = cmp::min(a1, cmp::min(a2, a3));
            }

            buffer[i1][i2]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transform;

    #[test]
    fn test1() {
        // Arrange
        let s1 = "a";
        let s2 = "a";
        let mut buffer = vec![vec![0_usize; 1]; 1];

        // Arrange
        let operations = transform(s1, s2, 0, 0, &mut buffer);

        // Assert
        assert_eq!(operations, 0);
    }

    #[test]
    fn test2() {
        // Arrange
        let s1 = "b";
        let s2 = "a";
        let mut buffer = vec![vec![0_usize; 1]; 1];

        // Arrange
        let operations = transform(s1, s2, 0, 0, &mut buffer);

        // Assert
        assert_eq!(operations, 1);
    }

    #[test]
    fn test3() {
        // Arrange
        let s1 = "bb";
        let s2 = "a";
        let mut buffer = vec![vec![0_usize; 2]; 2];

        // Arrange
        let operations = transform(s1, s2, 0, 0, &mut buffer);

        // Assert
        assert_eq!(operations, 2);
    }

    #[test]
    fn test4() {
        // Arrange
        let s1 = "This is a       test";
        let s2 = "This is another test";
        let mut buffer = vec![vec![0_usize; s2.len()]; s1.len()];

        // Arrange
        let operations = transform(s1, s2, 0, 0, &mut buffer);

        // Assert
        assert_eq!(operations, 6);
    }
}
