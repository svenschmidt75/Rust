// https://www.reddit.com/r/CodingProblems/comments/faewt5/day_520200227_problem_of_the_day_asked_by_palantir/

use std::collections::VecDeque;

fn next_permutation(input: &[i32]) -> Vec<i32> {
    let mut ordered = vec![0; input.len()];
    ordered.copy_from_slice(input);

    if input.len() < 2 {
        ordered
    } else {
        ordered.sort();
        next_permutation_internal(input, &ordered)
    }
}

fn next_permutation_internal(input: &[i32], sorted: &[i32]) -> Vec<i32> {
    match next_permutation_internal_2(input, sorted, 0) {
        (true, result) => result,
        _ => sorted.to_vec(),
    }
}

fn next_permutation_internal_2(input: &[i32], sorted: &[i32], index: usize) -> (bool, Vec<i32>) {
    if index == input.len() - 2 {
        // SS: only two left, swap order
        (
            input[index] < input[index + 1],
            vec![input[index + 1], input[index]],
        )
    } else {
        let (success, value) = next_permutation_internal_2(input, sorted, index + 1);
        if success == false {
            // SS: we need to replace input[index] with the next higher value
            // and append the remaining values in sorted order
            let mut ordered = input[index..].to_vec();
            ordered.sort();

            let index = ordered.iter().position(|&v| v == input[index]).unwrap();
            if index == ordered.len() - 1 {
                // SS: there is no higher value
                (false, input[index..].to_vec())
            } else {
                let mut result = vec![ordered[index + 1]];

                // SS: remove the next higher value
                ordered.remove(index + 1);

                // SS: append the remaining ones in sorted order
                ordered.into_iter().for_each(|v| result.push(v));
                (true, result)
            }
        } else {
            let mut result = input[index..index + 1].to_vec();
            value.into_iter().for_each(|v| result.push(v));
            (true, result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::next_permutation;

    #[test]
    fn test11() {
        // Arrange
        let input = [1, 2, 3];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [1, 3, 2].to_vec());
    }

    #[test]
    fn test21() {
        // Arrange
        let input = [2, 3, 4, 1];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [2, 4, 1, 3].to_vec());
    }

    #[test]
    fn test31() {
        // Arrange
        let input = [1, 2, 3, 4];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [1, 2, 4, 3].to_vec());
    }

    #[test]
    fn test41() {
        // Arrange
        let input = [2, 4, 3, 1];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [3, 1, 2, 4].to_vec());
    }

    #[test]
    fn test51() {
        // Arrange
        let input = [1, 3, 2];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [2, 1, 3].to_vec());
    }

    #[test]
    fn test61() {
        // Arrange
        let input = [3, 2, 1];

        // Act
        let result = next_permutation(&input);

        // Assert
        assert_eq!(result, [1, 2, 3].to_vec());
    }
}
