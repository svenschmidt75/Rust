use std::collections::HashSet;
use std::panic::resume_unwind;

// SS: Find first recurring number.

// Naive approach uses two nested loops, O(N^2) runtime complexity, but O(1) space

// HashSet: O(N) runtime complexity (we still have to loop over the array), at O(N)
// space complexity

// O(N^2), order-preserving
fn recurring_number_slow(array: &[u32]) -> Option<u32> {
    let mut min_dist = 2 * array.len();
    let mut min_item = 0;
    for i in 0..array.len() {
        let item1 = array[i];
        for j in (i + 1)..array.len() {
            let item2 = array[j];
            if item1 == item2 {
                if j + (j - i) < min_dist {
                    // SS: store a distance measure
                    min_dist = j + (j - i);
                    min_item = item1;
                    break;
                }
            }
        }
    }
    if min_dist < 2 * array.len() {
        Some(min_item)
    } else {
        None
    }
}

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
    use crate::{recurring_number, recurring_number_slow};

    #[test]
    fn boundary() {
        // Arrange
        let array = [];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, None);
    }

    #[test]
    fn test1() {
        // Arrange
        let array = [2, 5, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test1_2() {
        // Arrange
        let array = [2, 5, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test2() {
        // Arrange
        let array = [2, 1, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test2_2() {
        // Arrange
        let array = [2, 1, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test3() {
        // Arrange
        let array = [2, 3, 4, 5];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, None);
    }

    #[test]
    fn test3_2() {
        // Arrange
        let array = [2, 3, 4, 5];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, None);
    }

    #[test]
    fn test4() {
        // Arrange
        let array = [2, 5, 5, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test4_2() {
        // Arrange
        let array = [2, 5, 5, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test5() {
        // Arrange
        let array = [2, 5, 5, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test5_2() {
        // Arrange
        let array = [2, 5, 5, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test6() {
        // Arrange
        let array = [2, 5, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test6_2() {
        // Arrange
        let array = [2, 5, 1, 2, 3, 5, 1, 2, 4];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test7() {
        // Arrange
        let array = [2, 5, 7, 2, 7];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test7_2() {
        // Arrange
        let array = [2, 5, 7, 2, 7];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test8() {
        // Arrange
        let array = [2, 5, 1, 2, 7, 7];

        // Act
        let result = recurring_number(&array);

        // Act
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test8_2() {
        // Arrange
        let array = [2, 5, 1, 2, 7, 7];

        // Act
        let result = recurring_number_slow(&array);

        // Act
        assert_eq!(result, Some(2));
    }
}
