// Cracking the Coding Interview
// 6th ed, p. 91, ex. 1.9

fn is_rotation(s1: &str, s2: &str) -> bool {
    // SS: Turns out an easier solution is to concatenate s2 to itself,
    // then check if s1 is a substring of that concatenation...

    if s1.len() != s2.len() || s1.len() == 0 {
        false
    } else {
        // SS: assume
        // s1 = "erbottlewat";
        // s2 = "waterbottle";
        let s1_input = s1.chars().collect::<Vec<_>>();
        let s2_input = s2.chars().collect::<Vec<_>>();

        // SS: naive implementation, O(s1 * s2)
        for i in 0..s1.len() {
            let idx = s1.len() - 1 - i;
            if s1_input[idx] == s2_input[0] {
                if s1_input[idx..] == s2_input[0..=i] {
                    return is_substring(&s1[0..idx], &s2[i + 1..]);
                }
            }
        }

        // SS: both strings must be equal
        false
    }
}

fn is_substring(s1: &str, s2: &str) -> bool {
    // SS: s1 substring of s2?
    if s1.len() > s2.len() {
        false
    } else {
        let s1_input = s1.chars().collect::<Vec<_>>();
        let s2_input = s2.chars().collect::<Vec<_>>();
        let max_idx = s2.len() - s1.len() + 1;
        for i in 0..max_idx {
            if s1_input[..] == s2_input[i..] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let s1 = "waterbottle";
        let s2 = "waterbottle";

        // Act
        let result = is_rotation(s1, s2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let s1 = "erbottlewat";
        let s2 = "waterbottle";

        // Act
        let result = is_rotation(s1, s2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        // Arrange
        let s1 = "tlewaterbot";
        let s2 = "waterbottle";

        // Act
        let result = is_rotation(s1, s2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        // Arrange
        let s1 = "wattrbottle";
        let s2 = "tlewaterbot";

        // Act
        let result = is_rotation(s1, s2);

        // Assert
        assert_eq!(result, false);
    }
}
