
fn selection_sort(a: &mut [f64]) {
    // SS: Selection sort, ascending, an O(N^2) sorting algorithm,
    // in-place

    // SS: splits array in sorted and unsorted part
    // [0..partition_index] = sorted
    // [partition_index..] = unsorted
    let mut partition_index = 0;

    while partition_index < a.len() {
        let min_index = find_minimum(&mut a[partition_index..]);
        swap(&mut a[..], partition_index, partition_index + min_index);
        partition_index += 1;
    }
}

fn swap(data: &mut [f64], a: usize, b: usize) {
    let tmp = data[b];
    data[b] = data[a];
    data[a] = tmp;
}

fn find_minimum(a: &[f64]) -> usize {
    let mut min_index = std::usize::MAX;
    let mut min_value = std::f64::MAX;
    for i in 0..a.len() {
        let v = a[i];
        if v < min_value {
            min_value = v;
            min_index = i;
        }
    }
    min_index
}

#[cfg(test)]
mod tests {
    use crate::selection_sort;

    #[test]
    fn it_works() {
        // Arrange
        let mut a = [64.0, 25.0, 12.0, 22.0, 11.0];

        // Act
        selection_sort(&mut a);

        // Assert
        assert_eq!(a[0], 11.0);
        assert_eq!(a[1], 12.0);
        assert_eq!(a[2], 22.0);
        assert_eq!(a[3], 25.0);
        assert_eq!(a[4], 64.0);
    }
}
