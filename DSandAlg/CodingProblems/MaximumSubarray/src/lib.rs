use std::panic::resume_unwind;

// Kadane’s algorithm
fn maximum_subarray(array: &[i32]) -> i32 {
    if array.len() == 0 {
        0
    } else {
        let mut max_ending_here = 0;
        let mut max_ending_so_far = 0;
        let mut i = 0;

        while i < array.len() {
            let next = array[i];
            max_ending_here += next;
            if max_ending_here < 0 {
                max_ending_here = 0;
            }
            if max_ending_here > max_ending_so_far {
                max_ending_so_far = max_ending_here;
            }

            i += 1;
        }

        max_ending_so_far
    }
}


#[cfg(test)]
mod tests {
    use crate::maximum_subarray;

    #[test]
    fn test1() {
        // Arrange
        let input = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

        // Act
        let best_sum = maximum_subarray(&input);

        // Assert
        assert_eq!(best_sum, 6);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [-2, -3, 4, -1, -2, 1, 5, -3];

        // Act
        let best_sum = maximum_subarray(&input);

        // Assert
        assert_eq!(best_sum, 7);
    }
}
