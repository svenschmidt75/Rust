use std::collections::HashMap;

fn is_anagram(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        false
    } else {
        // SS: use frequency pattern
        let mut map1 = HashMap::new();
        for c in string1.chars() {
            let counter = map1.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut map2 = HashMap::new();
        for c in string2.chars() {
            let counter = map2.entry(c).or_insert(0);
            *counter += 1;
        }

        for c in string1.chars() {
            let freq1 = map1.entry(c).or_insert(0);
            let freq2 = map2.entry(c).or_insert(0);
            if freq1 != freq2 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::is_anagram;

    #[test]
    fn test1() {
        // Arrange
        let string1 = "";
        let string2 = "";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let string1 = "aaz";
        let string2 = "zza";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        // Arrange
        let string1 = "anagram";
        let string2 = "nagaram";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        // Arrange
        let string1 = "rat";
        let string2 = "car";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        // Arrange
        let string1 = "awesome";
        let string2 = "awesom";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        // Arrange
        let string1 = "qwerty";
        let string2 = "qeywrt";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test7() {
        // Arrange
        let string1 = "texttwisttime";
        let string2 = "timetwisttext";

        // Act
        let result = is_anagram(string1, string2);

        // Assert
        assert_eq!(result, true);
    }
}
