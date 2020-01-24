fn find_subsets(array: &[u64]) -> Vec<(Vec<u64>, Vec<u64>)> {
    let mut solutions = vec![];
    find_subsets_rec(array.len() - 1, array, &[], &[array[0]], &mut solutions);
    solutions
}

fn find_subsets_rec(
    n: usize,
    array: &[u64],
    left: &[u64],
    right: &[u64],
    solutions: &mut Vec<(Vec<u64>, Vec<u64>)>,
) {
    // SS: recursion base case
    if n == 0 {
        let left_sum: u64 = left.iter().sum();
        let right_sum: u64 = right.iter().sum();
        if left_sum == right_sum {
            solutions.push((left.to_vec(), right.to_vec()));
        }
    } else {
        // SS: add next number
        let idx = array.len() - n;
        let value = array[idx];

        let mut new_left = left.to_vec();
        new_left.push(value);
        find_subsets_rec(n - 1, array, &new_left[..], right, solutions);

        let mut new_right = right.to_vec();
        new_right.push(value);
        find_subsets_rec(n - 1, array, left, &new_right[..], solutions);
    }
}

#[cfg(test)]
mod tests {
    use crate::find_subsets;

    #[test]
    fn test1() {
        // Arrange
        let array = [5, 10, 15];

        // Act
        let solutions = find_subsets(&array);

        // Assert
        assert_eq!(1, solutions.len());
    }

    #[test]
    fn test2() {
        // Arrange
        let array = [15, 5, 20, 10, 35, 15, 10];

        // Act
        let solutions = find_subsets(&array);

        // Assert
        assert_eq!(4, solutions.len());
    }
}
