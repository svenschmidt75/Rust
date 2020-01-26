// https://leetcode.com/problems/power-of-three/

fn power_of_three1(n: i64) -> bool {
    // SS: n negative?

    if n == 0 {
        false
    } else {
        let mut value = 1;
        while value < n {
            value *= 3;
        }
        value == n
    }
}

#[cfg(test)]
mod tests {
    use crate::power_of_three1;

    #[test]
    fn test1() {
        // Arrange
        let n = 27;

        // Act
        let is_power_of_three = power_of_three1(n);

        // Assert
        assert_eq!(is_power_of_three, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let n = 0;

        // Act
        let is_power_of_three = power_of_three1(n);

        // Assert
        assert_eq!(is_power_of_three, false);
    }

    #[test]
    fn test3() {
        // Arrange
        let n = 9;

        // Act
        let is_power_of_three = power_of_three1(n);

        // Assert
        assert_eq!(is_power_of_three, true);
    }

    #[test]
    fn test4() {
        // Arrange
        let n = 45;

        // Act
        let is_power_of_three = power_of_three1(n);

        // Assert
        assert_eq!(is_power_of_three, false);
    }
}
