// Largest sum in subarray
// Uses Kadane's algorithm, running at O(n)
// Dynamic programing technique

use std::cmp;

fn solve(input: &[i32]) -> i32 {
    if input.len() < 2 {
        input[0]
    } else {
        let mut largest_sum = input[0];
        let mut sum = largest_sum;

        for i in 1..input.len() {
            let item = input[i];
            sum = cmp::max(item, sum + item);
            largest_sum = cmp::max(largest_sum, sum);
        }

        largest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let input = [1, -3, 2, 1, -1];

        // Act
        let sum = solve(&input);

        // Assert
        assert_eq!(sum, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [
            0, 13, -3, -25, 20, -3, -16, -21, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];

        // Act
        let sum = solve(&input);

        // Assert
        assert_eq!(sum, 43);
    }
}
