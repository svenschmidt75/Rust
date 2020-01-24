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

fn is_anagram2(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        false
    } else {
        // SS: use frequency pattern
        let str1 = string1.chars().collect::<Vec<_>>();
        let str2 = string2.chars().collect::<Vec<_>>();

        let mut zeros: i64 = 0;

        let mut map = HashMap::new();
        for i in 0..string1.len() {
            let c1 = str1[i];
            let freq = map.entry(c1).or_insert(0);
            *freq += 1;

            if *freq == 0 {
                zeros += 1;
            } else {
                zeros -= 1;
            }

            let c2 = str2[i];
            let freq = map.entry(c2).or_insert(0);
            *freq -= 1;

            if *freq == 0 {
                zeros += 1;
            } else {
                zeros -= 1;
            }
        }

        zeros == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::{is_anagram, is_anagram2};

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
        let result = is_anagram2(string1, string2);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        // Arrange
        let string1 = "rat";
        let string2 = "car";

        // Act
        let result = is_anagram2(string1, string2);

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
