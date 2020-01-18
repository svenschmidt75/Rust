fn insertion_sort(a: &mut [f64]) {
    // SS: Insertion sort, in-place, with O(N^2) runtime complexity,
    // but O(1) memory
    for i in 1..a.len() {
        let cnt = i;
        let mut j = 0;
        while j < cnt && a[i - j] < a[i - j - 1] {
            swap(a, i - j, i - j - 1);
            j += 1;
        }
    }
}

fn swap(array: &mut [f64], a: usize, b: usize) {
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
}

fn insertion_sort_2(a: &mut [f64]) {
    // SS: Insertion sort, in-place, with O(N^2) runtime complexity,
    // but O(1) memory
    for i in 1..a.len() {
        if a[i - 1] > a[i] {
            let value = a[i];
            let insertion_index = find_index(&a, value, i);
            shift(&mut a[..], insertion_index, i);
            a[insertion_index] = value;
        }
    }
}

fn find_index(a: &[f64], value: f64, max_index: usize) -> usize {
    let mut i = 0;
    while i < max_index {
        let v = a[i];
        if v > value {
            return i;
        }
        i += 1;
    }
    panic!("unexpected")
}

fn shift(a: &mut [f64], start: usize, end: usize) {
    // SS: shift elements a[start..end] one to the right
    // a=[64, 25, 12, 22, 11], start=1, end=3 => a=[64, 25, 25, 12, 11]
    for i in start..end {
        // SS: iterate from end to beginning
        let idx = end - i + start;
        a[idx] = a[idx - 1];
    }
}

#[cfg(test)]
mod tests {
    use crate::{insertion_sort, insertion_sort_2};

    #[test]
    fn test_1() {
        // Arrange
        let mut a = [64.0, 25.0, 12.0, 22.0, 11.0];

        // Act
        insertion_sort(&mut a);

        // Assert
        assert_eq!(a[0], 11.0);
        assert_eq!(a[1], 12.0);
        assert_eq!(a[2], 22.0);
        assert_eq!(a[3], 25.0);
        assert_eq!(a[4], 64.0);
    }

    #[test]
    fn test_2() {
        // Arrange
        let mut a = [64.0, 25.0, 12.0, 22.0, 11.0];

        // Act
        insertion_sort_2(&mut a);

        // Assert
        assert_eq!(a[0], 11.0);
        assert_eq!(a[1], 12.0);
        assert_eq!(a[2], 22.0);
        assert_eq!(a[3], 25.0);
        assert_eq!(a[4], 64.0);
    }
}
