fn get_digit(num: i64, pos: usize) -> i64 {
    let p1 = 10i64.pow(pos as u32 + 1);
    let t1 = num / p1;
    let t2 = num - t1 * p1;

    let p2 = 10i64.pow(pos as u32);
    let digit = t2 / p2;
    digit.abs()
}

fn number_of_digits(num: i64) -> i64 {
    let mut cnt = 0;
    let mut base = 1;
    loop {
        let d = num / base;
        if d == 0 {
            break;
        }
        cnt += 1;
        base *= 10;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_digits1() {
        // Arrange
        let number = 94671;

        // Act
        let digits = number_of_digits(number);

        // Assert
        assert_eq!(5, digits);
    }

    #[test]
    fn test_get_digit_negative() {
        // Arrange
        let number = -94671;

        // Act
        let digit = get_digit(number, 2);

        // Assert
        assert_eq!(6, digit);
    }

    #[test]
    fn test_get_digit1() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 2);

        // Assert
        assert_eq!(6, digit);
    }

    #[test]
    fn test_get_digit2() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 0);

        // Assert
        assert_eq!(1, digit);
    }

    #[test]
    fn test_get_digit3() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 3);

        // Assert
        assert_eq!(4, digit);
    }

    #[test]
    fn test_get_digit4() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 4);

        // Assert
        assert_eq!(9, digit);
    }

    #[test]
    fn test_get_digit5() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 5);

        // Assert
        assert_eq!(0, digit);
    }
}
