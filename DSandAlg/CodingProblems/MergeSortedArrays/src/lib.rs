
// runtime complexity is O(m + n)
fn merge(array1: &[u64], array2: &[u64]) -> Vec<u64> {
    let mut total_sorted = vec![0; array1.len() + array2.len()];
    let mut i =0;
    let mut j =0;
    let mut k = 0;
    while i < array1.len() && j < array2.len() {
        if array1[i] < array2[j] {
            total_sorted[k] = array1[i];
            i += 1;
        } else {
            total_sorted[k] = array2[j];
            j += 1;
        }
        k += 1;
    }

    while i < array1.len() {
        total_sorted[k] = array1[i];
        i += 1;
        k += 1;
    }

    while k < array1.len() {
        total_sorted[k] = array2[j];
        j += 1;
        k += 1;
    }

    total_sorted
}

#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn boundary_conditions_1() {
        // Arrange
        let array1 = [];
        let array2 = [];

        // Act
        let sorted = merge(&array1, &array2);

        // Act
        assert_eq!(0, sorted.len());
    }

    #[test]
    fn boundary_conditions_2() {
        // Arrange
        let array1 = [0, 3, 4, 31];
        let array2 = [];

        // Act
        let sorted = merge(&array1, &array2);

        // Act
        assert_eq!((&[0, 3, 4, 31]).to_vec(), sorted);
    }

    #[test]
    fn general_case() {
        // Arrange
        let array1 = [0, 3, 4, 31];
        let array2 = [4, 6, 30];

        // Act
        let sorted = merge(&array1, &array2);

        // Act
        assert_eq!((&[0, 3, 4, 4, 6, 30, 31]).to_vec(), sorted);
    }
}
