// https://leetcode.com/problems/excel-sheet-column-number/

fn column_number(column: &str) -> u64 {
    let mut pos = 0;
    let mut col = 0;
    for c in column.chars().rev() {
        let n = c as u8;
        let i = (n.checked_sub('A' as u8).unwrap() + 1) as u64;
        col += i * 26_u64.pow(pos);
        pos += 1;
    }
    col
}

#[cfg(test)]
mod tests {
    use crate::column_number;

    #[test]
    fn test1() {
        // Arrange
        let column = "A";

        // Act
        let col = column_number(column);

        // Assert
        assert_eq!(col, 1);
    }

    #[test]
    fn test2() {
        // Arrange
        let column = "AA";

        // Act
        let col = column_number(column);

        // Assert
        assert_eq!(col, 27);
    }

    #[test]
    fn test3() {
        // Arrange
        let column = "AB";

        // Act
        let col = column_number(column);

        // Assert
        assert_eq!(col, 28);
    }

    #[test]
    fn test4() {
        // Arrange
        let column = "ZY";

        // Act
        let col = column_number(column);

        // Assert
        assert_eq!(col, 701);
    }
}
