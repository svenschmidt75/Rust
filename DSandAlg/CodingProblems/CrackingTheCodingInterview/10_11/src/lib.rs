// Cracking the Coding Interview
// 6th ed, p. 151, ex. 10.11

fn peaks_and_valleys_1(input: &[u64]) -> Vec<u64> {
    // SS: O(N log N)
    let mut sorted = input.to_owned();
    sorted.sort();

    let mut result = vec![];

    // SS: O(N)
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

fn peaks_and_valleys_2(input: &[u64]) -> Vec<u64> {
    if input.len() < 3 {
        input.to_vec()
    } else {
        let mut result = input.to_owned();

        // SS: O(N) solution

        let mut i = 0;
        while i < (result.len() - 2) {
            // SS: put the largest at index i
            if result[i] < result[i + 1] {
                result.swap(i, i + 1);
            }
            if result[i] < result[i + 2] {
                result.swap(i, i + 2);
            }

            // SS: put the middle one at index i + 1
            if result[i + 1] > result[i + 2] {
                result.swap(i + 1, i + 2);
            }

            i += 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let input = [5, 3, 1, 2, 3];

        // Act
        let result = peaks_and_valleys_1(&input);

        // Assert
        assert_eq!(result, vec![5, 1, 3, 2, 3]);
    }

    #[test]
    fn test12() {
        // Arrange
        let input = [5, 8, 6, 2, 3, 4, 6];

        // Act
        let result = peaks_and_valleys_1(&input);

        // Assert
        assert_eq!(result, vec![8, 2, 6, 3, 6, 4, 5]);
    }

    #[test]
    fn test21() {
        // Arrange
        let input = [5, 3, 1, 2, 3];

        // Act
        let result = peaks_and_valleys_2(&input);

        // Assert
        assert_eq!(result, vec![5, 1, 3, 2, 3]);
    }

    #[test]
    fn test22() {
        // Arrange
        let input = [5, 8, 6, 2, 3, 4, 6];

        // Act
        let result = peaks_and_valleys_2(&input);

        // Assert
        assert_eq!(result, vec![8, 5, 6, 2, 6, 3, 4]);
    }
}
