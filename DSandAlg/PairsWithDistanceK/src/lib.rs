// Cracking the Coding Interview
// 6th ed, page 67

fn pairs_with_distance_k(input: &[u32], k: u32) -> Vec<(u32, u32)> {
    // SS: sort array at O(n log n)
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    let mut result = vec![];

    // SS: O(n)
    let mut i = 0;
    let mut j = 1;
    while j < input.len() {
        let v1 = sorted_input[i];
        let v2 = sorted_input[j];
        let sum = v2 - v1;
        if sum < k {
            j += 1;
        } else if sum > k {
            i += 1;
            j = i + 1;
        } else {
            result.push((v1, v2));
            i += 1;
            j = i + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::pairs_with_distance_k;

    #[test]
    fn it_works() {
        // Arrange
        let input = [1, 7, 5, 9, 2, 12, 3];

        // Act
        let pairs = pairs_with_distance_k(&input, 2);

        // Assert
        assert_eq!(pairs.len(), 4);
    }
}
