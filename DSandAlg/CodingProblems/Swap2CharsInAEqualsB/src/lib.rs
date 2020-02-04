// https://www.reddit.com/r/AlgoExpert/comments/evoh1a/day_6_20200129_problem_of_the_day_asked_by_airbnb/

fn solve1(a: &str, b: &str) -> bool {
    // SS: runtime complexity: O(k^2), where k = length(a) = length(b)
    if a.len() < 2 || a.len() != b.len() {
        false
    } else {
        let avec = a.chars().into_iter().collect::<Vec<_>>();
        let bvec = b.chars().into_iter().collect::<Vec<_>>();

        // SS: could use pointers here instead of physically swapping...
        for i in 0..=(a.len() - 2) {
            if avec[i] == bvec[i + 1] && avec[i + 1] == bvec[i] && &avec[i + 2..] == &bvec[i + 2..]
            {
                return true;
            }
        }

        false
    }
}

fn solve2(a: &str, b: &str) -> bool {
    // SS: runtime complexity: O(k), where k = length(a) = length(b)
    if a.len() < 2 || a.len() != b.len() {
        false
    } else {
        let avec = a.chars().into_iter().collect::<Vec<_>>();
        let bvec = b.chars().into_iter().collect::<Vec<_>>();

        let mut valid = false;

        // SS: could use pointers here instead of physically swapping...
        for i in 0..=(a.len() - 2) {
            if avec[i] == bvec[i] && avec[i + 1] == bvec[i + 1] {
                if avec[i] == avec[i + 1] {
                    // SS: can be swapped
                    valid = true;
                } else {
                    // SS: chars are different, cannot be swapped
                    valid = false;
                }
            } else {
                // SS: chars are different, check swap and compare
                if avec[i] == bvec[i + 1]
                    && avec[i + 1] == bvec[i]
                    && &avec[i + 2..] == &bvec[i + 2..]
                {
                    return true;
                } else {
                    valid = false;
                }
            }
        }

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let a = "ab";
        let b = "ba";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test12() {
        // Arrange
        let a = "ab";
        let b = "ab";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test13() {
        // Arrange
        let a = "aa";
        let b = "aa";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test14() {
        // Arrange
        let a = "aaaaaaabc";
        let b = "aaaaaaacb";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test15() {
        // Arrange
        let a = "abbc";
        let b = "abcb";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }
    #[test]
    fn test21() {
        // Arrange
        let a = "ab";
        let b = "ba";

        // Act
        let result = solve2(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test22() {
        // Arrange
        let a = "ab";
        let b = "ab";

        // Act
        let result = solve2(a, b);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test23() {
        // Arrange
        let a = "aa";
        let b = "aa";

        // Act
        let result = solve2(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test24() {
        // Arrange
        let a = "aaaaaaabc";
        let b = "aaaaaaacb";

        // Act
        let result = solve2(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test25() {
        // Arrange
        let a = "abbc";
        let b = "abcb";

        // Act
        let result = solve2(a, b);

        // Assert
        assert_eq!(result, true);
    }
}
