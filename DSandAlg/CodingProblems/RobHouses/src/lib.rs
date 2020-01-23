use std::cmp;

fn rob_houses(house_values: &[u64]) -> u64 {
    // O(N) solution

    if house_values.is_empty() {
        return 0;
    }

    let mut odd = 0;
    let mut even = 0;

    for i in 0..house_values.len() {
        let house_value = house_values[i];
        match i % 2 {
            0 => even += house_value,
            _ => odd += house_value,
        }
    }

    cmp::max(even, odd)
}

#[cfg(test)]
mod tests {
    use crate::rob_houses;

    #[test]
    fn test1() {
        // Arrange
        let house_values = [1, 2, 3, 1];

        // Act
        let profit = rob_houses(&house_values);

        // Assert
        assert_eq!(profit, 4);
    }

    #[test]
    fn test2() {
        // Arrange
        let house_values = [2, 7, 9, 3, 1];

        // Act
        let profit = rob_houses(&house_values);

        // Assert
        assert_eq!(profit, 12);
    }

    #[test]
    fn test3() {
        // Arrange
        let house_values = [6, 7, 1, 3, 8, 2, 4];

        // Act
        let profit = rob_houses(&house_values);

        // Assert
        assert_eq!(profit, 19);
    }
}
