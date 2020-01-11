
fn rotate_array_1(array: &mut [i32], k: usize) {
    // SS: rotate array in O(k*n)=O(n) runtime and O(1) space complexity
    for r in 0..k {
        // SS: save last element
        let tmp = array[array.len() - 1];
        for i in 0..(array.len() - 1) {
            array[array.len() - 1 - i] = array[array.len() - 1 - i - 1];
        }
        array[0] = tmp;
    }
}

fn rotate_array_2(array: &mut [i32], k: usize) {
    // SS: rotate array in O(n) runtime and O(n) space complexity
    let copy = array.to_vec();
    let start_index = array.len() - k;
    for i in 0..array.len() {
        let idx = (start_index + i) % array.len();
        array[i] = copy[idx];
    }
}

fn swap(array: &mut [i32], a: usize, b: usize) {
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
}

#[cfg(test)]
mod tests {
    use crate::{rotate_array_1, rotate_array_2};

    #[test]
    fn test_rotate_array_1() {
        // Arrange
        let mut array = [1, 2, 3, 4, 5, 6, 7];

        // Act
        rotate_array_1(&mut array, 3);

        // Assert
        assert_eq!(array, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_array_2() {
        // Arrange
        let mut array = [1, 2, 3, 4, 5, 6, 7];

        // Act
        rotate_array_2(&mut array, 3);

        // Assert
        assert_eq!(array, [5, 6, 7, 1, 2, 3, 4]);
    }
}
