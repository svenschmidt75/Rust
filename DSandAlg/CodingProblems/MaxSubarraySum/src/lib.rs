fn max_subarray_sum(array: &[u64], width: u64) -> u64 {
    if array.len() < width as usize {
        0
    } else {
        let mut sum = 0;
        for i in 0..width {
            sum += array[i as usize];
        }
        let mut max_sum = sum;

        let mut i = 0;
        let mut j = width as usize;
        while j < array.len() {
            sum -= array[i];
            i += 1;
            sum += array[j];
            if sum > max_sum {
                max_sum = sum;
            }
            j += 1;
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::max_subarray_sum;

    #[test]
    fn test1() {
        // Arrange
        let array = [1, 2, 5, 2, 8, 1, 5];

        // Act
        let max_sum = max_subarray_sum(&array, 2);

        // Assert
        assert_eq!(max_sum, 10);
    }

    #[test]
    fn test2() {
        // Arrange
        let array = [1, 2, 5, 2, 8, 1, 5];

        // Act
        let max_sum = max_subarray_sum(&array, 4);

        // Assert
        assert_eq!(max_sum, 17);
    }

    #[test]
    fn test3() {
        // Arrange
        let array = [4, 2, 1, 6];

        // Act
        let max_sum = max_subarray_sum(&array, 1);

        // Assert
        assert_eq!(max_sum, 6);
    }

    #[test]
    fn test4() {
        // Arrange
        let array = [4, 2, 1, 6, 2];

        // Act
        let max_sum = max_subarray_sum(&array, 4);

        // Assert
        assert_eq!(max_sum, 13);
    }

    #[test]
    fn test5() {
        // Arrange
        let array = [];

        // Act
        let max_sum = max_subarray_sum(&array, 4);

        // Assert
        assert_eq!(max_sum, 0);
    }
}
