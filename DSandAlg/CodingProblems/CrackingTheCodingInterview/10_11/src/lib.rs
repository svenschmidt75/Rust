// Cracking the Coding Interview
// 6th ed, p. 151, ex. 10.11

fn peaks_and_valleys(input: &[u64]) -> Vec<u64> {
    // SS: O(N log N)
    let mut sorted = input.to_owned();
    sorted.sort();

    let mut result = vec![];

    for i in 0..(sorted.len() / 2) {
        let value = sorted[sorted.len() - 1 - i];
        result.push(value);

        let value = sorted[i];
        result.push(value);
    }

    if sorted.len() % 2 == 1 {
        result.push(sorted[sorted.len() / 2]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let input = [5, 3, 1, 2, 3];

        // Act
        let result = peaks_and_valleys(&input);

        // Assert
        assert_eq!(result, vec![5, 1, 3, 2, 3]);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [5, 8, 6, 2, 3, 4, 6];

        // Act
        let result = peaks_and_valleys(&input);

        // Assert
        assert_eq!(result, vec![8, 2, 6, 3, 6, 4, 5]);
    }
}
