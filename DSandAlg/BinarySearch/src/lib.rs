fn binary_search_internal(array: &[u64], value: u64, min: usize, max: usize) -> Option<usize> {
    // SS: interval is [min, max), i.e. max is exclusive
    let interval_length = max - min;

    // SS: This is a recursive algorithm, so we need a base case that
    // terminates the recursion...

    // SS: base case
    if interval_length == 1 {
        if array[min] == value {
            Some(min)
        } else {
            None
        }
    } else {
        // SS: recursive branch
        let middle = min + interval_length / 2;
        let v = array[middle];
        if v <= value {
            binary_search_internal(array, value, middle, max)
        } else {
            binary_search_internal(array, value, min, middle)
        }
    }
}

fn binary_search(array: &[u64], value: u64) -> Option<usize> {
    binary_search_internal(array, value, 0, array.len())
}

#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn not_found() {
        // Arrange
        let a = [1, 2, 4, 5, 9, 13, 16, 19];

        // Act
        let found = binary_search(&a, 3);

        // Assert
        assert_eq!(None, found);
    }

    #[test]
    fn found() {
        // Arrange
        let a = [1, 2, 4, 5, 9, 13, 16, 19];

        // Act
        let found = binary_search(&a, 4);

        // Assert
        assert_eq!(Some(2), found);
    }
}
