use std::collections::HashSet;
use std::panic::resume_unwind;

// SS: Find first recurring number.

// Naive approach uses two nested loops, O(N^2) runtime complexity, but O(1) space

// HashSet: O(N) runtime complexity (we still have to loop over the array), at O(N)
// space complexity

fn recurring_number(array: &[u32]) -> Option<u32> {
    let mut hash_set = HashSet::with_capacity(array.len());
    for i in array {
        if hash_set.contains(i) {
            return Some(*i);
        } else {
            hash_set.insert(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{recurring_number, recurring_number_slow, recurring_number_slow_2};

    #[test]
    fn boundary() {
        // Arrange
        let array = [];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(None, result);
    }

    #[test]
    fn test1() {
        // Arrange
        let array = [2, 5, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(Some(2), result);
    }

    #[test]
    fn test2() {
        // Arrange
        let array = [2, 1, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(Some(1), result);
    }

    #[test]
    fn test3() {
        // Arrange
        let array = [2, 3, 4, 5];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(None, result);
    }

    #[test]
    fn test4() {
        // Arrange
        let array = [2, 5, 5, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(Some(5), result);
    }

}
