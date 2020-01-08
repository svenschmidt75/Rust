// https://leetcode.com/problems/move-zeroes/description/
// https://tutorialspoint.dev/data-structure/arrays/move-zeroes-end-array

fn move_zeros_to_end(array: &mut [i32]) {
    if array.len() == 0 {
        return;
    }
    let mut end = array.len() - 1;
    let mut i = 0;
    while i < end {
        if array[i] == 0 {
            // SS: shift
            for j in i..end {
                array[j] = array[j + 1];
            }
            array[end] = 0;
            end -= 1;
        } else {
            i += 1;
        }
    }
}

// without shifting each time, but still O(n^2)
fn move_zeros_to_end_2(array: &mut [i32]) {
    let mut i = 0;
    while i < array.len() {
        // SS: this is O(n)
        if array[i] == 0 {
            // SS: this is O(n)
            let mut j = find_next_nonzero(&array, i + 1);
            if j < array.len() {
                array[i] = array[j];
                array[j] = 0;
            }
        }

        i += 1;
    }
}

fn find_next_nonzero(array: &[i32], from_index: usize) -> usize {
    let mut i = from_index;
    while i < array.len() && array[i] == 0 {
        i += 1;
    }
    i
}

// this is O(n)
fn move_zeros_to_end_3(array: &mut [i32]) {
    let mut count = 0;
    for i in 0..array.len() {
        if array[i] != 0 {
            array[count] = array[i];
            count += 1;
        }
    }

    for i in count..array.len() {
        array[i] = 0;
    }
}

/*
    1. Quicksort O(N log N)
    2. reverse array O(N)
      is O(N long N + N) = O(N log N)

    1. Reverse Quicksort, O(N log N)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary() {
        // Arrange
        let mut input = [];

        // Act
        move_zeros_to_end(&mut input);

        // Assert
        let expected: [i32; 0] = [];
        assert_eq!(expected, input);
    }

    #[test]
    fn test_boundary2() {
        // Arrange
        let mut input = [];

        // Act
        move_zeros_to_end_2(&mut input);

        // Assert
        let expected: [i32; 0] = [];
        assert_eq!(expected, input);
    }

    #[test]
    fn test_1_1() {
        // Arrange
        let mut input = [0, 1, 0, 3, 12];

        // Act
        move_zeros_to_end(&mut input);

        // Assert
        assert_eq!([1, 3, 12, 0, 0], input);
    }

    #[test]
    fn test_1_2() {
        // Arrange
        let mut input = [1, 2, 0, 4, 3, 0, 5, 0];

        // Act
        move_zeros_to_end(&mut input);

        // Assert
        assert_eq!([1, 2, 4, 3, 5, 0, 0, 0], input);
    }

    #[test]
    fn test_1_3() {
        // Arrange
        let mut input = [1, 2, 0, 0, 0, 3, 6];

        // Act
        move_zeros_to_end(&mut input);

        // Assert
        assert_eq!([1, 2, 3, 6, 0, 0, 0], input);
    }

    #[test]
    fn test_2_1() {
        // Arrange
        let mut input = [0, 1, 0, 3, 12];

        // Act
        move_zeros_to_end_2(&mut input);

        // Assert
        assert_eq!([1, 3, 12, 0, 0], input);
    }

    #[test]
    fn test_2_2() {
        // Arrange
        let mut input = [1, 2, 0, 4, 3, 0, 5, 0];

        // Act
        move_zeros_to_end_2(&mut input);

        // Assert
        assert_eq!([1, 2, 4, 3, 5, 0, 0, 0], input);
    }

    #[test]
    fn test_2_3() {
        // Arrange
        let mut input = [1, 2, 0, 0, 0, 3, 6];

        // Act
        move_zeros_to_end_2(&mut input);

        // Assert
        assert_eq!([1, 2, 3, 6, 0, 0, 0], input);
    }

    #[test]
    fn test_3_1() {
        // Arrange
        let mut input = [0, 1, 0, 3, 12];

        // Act
        move_zeros_to_end_3(&mut input);

        // Assert
        assert_eq!([1, 3, 12, 0, 0], input);
    }

    #[test]
    fn test_3_2() {
        // Arrange
        let mut input = [1, 2, 0, 4, 3, 0, 5, 0];

        // Act
        move_zeros_to_end_3(&mut input);

        // Assert
        assert_eq!([1, 2, 4, 3, 5, 0, 0, 0], input);
    }

    #[test]
    fn test_3_3() {
        // Arrange
        let mut input = [1, 2, 0, 0, 0, 3, 6];

        // Act
        move_zeros_to_end_3(&mut input);

        // Assert
        assert_eq!([1, 2, 3, 6, 0, 0, 0], input);
    }
}
