// Sliding Window Technique + 4 Questions - Algorithms
// https://www.youtube.com/watch?v=jM2dhDPYMQM

fn subarray_adds_up_to_number(input: &[i32], sum: i32) {
    // SS: use sliding window technique
    let mut i = 0;
    let mut j = 0;
    let mut running_sum = 0;
    while i < input.len() && j < input.len() {
        if running_sum + input[j] < sum {
            running_sum += input[j];
            j += 1;
        } else if running_sum + input[j] > sum {
            running_sum -= input[i];
            i += 1;
        } else {
            println!("Found subarray: {} - {}", i, j);
            j += 1;
            i = j;
            running_sum = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarrays_that_add_up_to_9() {
        // Arrange
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Act
        subarray_adds_up_to_number(&input, 9);

        //        assert_eq!(2 + 2, 4);
    }
}
