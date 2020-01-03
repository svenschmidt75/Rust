fn partition(array: &mut [u64], min: usize, max: usize) -> usize {
    // SS: by convention, we use the last element in the range as pivot
    let mut pivot_index = max - 1;

    let mut current_index = min;
    while current_index < pivot_index {
        if array[current_index] > array[pivot_index] {
            // SS: assign to local vars so nothing gets overwritten
            // when the current_index == pivot_index - 1
            swap(array, current_index, pivot_index);
            swap(array, current_index, pivot_index - 1);
            pivot_index = pivot_index - 1;
        } else {
            current_index = current_index + 1;
        }
    }
    pivot_index
}

fn swap(array: &mut [u64], a: usize, b: usize) {
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
}

fn quicksort_internal(array: &mut [u64], min: usize, max: usize) {
    // SS: interval is [min, max), i.e. exclusive

    // SS: base case for recursion
    if max - min <= 1 {
        // SS: done, nothing to do
        return;
    }

    // SS: recursive part

    // SS: partition array
    let pivot_index = partition(array, min, max);

    // SS: sort subarrays
    quicksort_internal(array, min, pivot_index);
    quicksort_internal(array, pivot_index + 1, max);
}

fn quicksort(array: &mut [u64]) {
    quicksort_internal(array, 0, array.len())
}

#[cfg(test)]
mod tests {
    use crate::quicksort;

    #[test]
    fn sort() {
        // Arrange
        let mut unsorted_array = [1, 16, 5, 13, 9, 19, 8];

        // Act
        quicksort(&mut unsorted_array);

        // Assert
        assert_eq!(unsorted_array, [1, 5, 8, 9, 13, 16, 19]);
    }
}
