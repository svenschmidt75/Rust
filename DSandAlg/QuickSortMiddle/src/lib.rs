fn swap(array: &mut [u64], a: usize, b: usize) {
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
}

fn partition(array: &mut [u64], min: usize, max: usize) -> usize {
    // SS: partition strategy is to use the middle index
    let mut pivot_index = (max + min) / 2;

    let mut i = min;
    while i < pivot_index {
        if array[i] > array[pivot_index] {
            swap(array, i, pivot_index);
            swap(array, i, pivot_index - 1);
            pivot_index = pivot_index - 1;
        } else {
            i += 1;
        }
    }

    let mut i = pivot_index + 1;
    while i < max {
        if array[i] < array[pivot_index] {
            swap(array, i, pivot_index);
            swap(array, i, pivot_index + 1);
            pivot_index = pivot_index + 1;
        }
        i += 1;
    }

    pivot_index
}

fn quicksort_internal(array: &mut [u64], min: usize, max: usize) {
    // SS: base case for recursion
    if max - min <= 1 {
        return;
    }

    // SS: recursive part
    let partition_index = partition(array, min, max);

    quicksort_internal(array, min, partition_index);
    quicksort_internal(array, partition_index + 1, max);
}

fn quicksort(array: &mut [u64]) {
    quicksort_internal(array, 0, array.len())
}

#[cfg(test)]
mod tests {
    use crate::quicksort;

    #[test]
    fn sort1() {
        // Arrange
        let mut unsorted_array = [1, 16, 5, 13, 9, 19, 8];

        // Act
        quicksort(&mut unsorted_array);

        // Assert
        assert_eq!(unsorted_array, [1, 5, 8, 9, 13, 16, 19]);
    }

    #[test]
    fn sort2() {
        // Arrange
        let mut unsorted_array = [0, 1, 2, 7, 3, 10, 8];

        // Act
        quicksort(&mut unsorted_array);

        // Assert
        assert_eq!(unsorted_array, [0, 1, 2, 3, 7, 8, 10]);
    }
}
