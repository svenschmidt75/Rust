/* Given an array of integers and the number of partitions to generate,
 * find the max. minimum sum of all the possible partitions.
 * 1-D array of integers, you need to partition it into contiguous pieces
 * and optimize the minimal sum.
 *
 * Input: Array of size N, split in k pieces.
 * 10  1  2  | 5  7 | 8  10
 * Sums: p1=13, p2=12, p3=18, so 12 is the minimum
 */

use std::cmp;

fn brute_force(input: &[i64], k: i64) -> i64 {
    // SS: this approach is O(N^{k-1})
    let partition_sums = vec![];
    let max_min_sum = generate_partitions(&input, partition_sums, 0, k - 1);
    max_min_sum
}

fn generate_partitions(
    input: &[i64],
    mut partition_sums: Vec<i64>,
    input_index: usize,
    n: i64,
) -> i64 {
    // SS: base case
    if n == 0 {
        // SS: find sum of last partition
        let ps = input[input_index..].iter().sum();
        partition_sums.push(ps);

        // SS: find minimum partition sum
        let &min_sum = partition_sums.iter().min().unwrap();
        min_sum
    } else {
        let mut global_min_sum = std::i64::MIN;
        for i in (input_index + 1)..input.len() {
            // SS: calc sum of partition
            let ps = input[input_index..i].iter().sum::<i64>();

            let mut sums = partition_sums.clone();
            sums.push(ps);
            let min_sum = generate_partitions(input, sums, i, n - 1);
            global_min_sum = cmp::max(min_sum, global_min_sum);
        }
        global_min_sum
    }
}


fn optimal_solution(input: &[i64], k: i64) -> i64 {
    // SS: O(N log N) solution utilizing binary search

    0
}

fn generate_and_validate(input: &input[i64], partition: (usize, usize), sum: i64) -> bool {
    


}

fn find_partition_with_sum(input: &[i64], sum: i64) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..input.len() {
        let mut s = input[i];
        let mut j = i + 1;
        while j < input.len() && s < sum {
            s += input[j];
            j += 1;
        }
        if j == input.len() || s > sum{
            continue;
        }
        assert_eq!(sum, s);

        // SS: [start, end)
        result.push((i, j));
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::{brute_force, find_partition_with_sum};

    #[test]
    fn test_brute_force() {
        // Arrange
        let input = [10, 1, 2, 5, 7, 8, 10];

        // Act
        let max_min_sum = brute_force(&input, 3);

        // Assert
        assert_eq!(max_min_sum, 12);
    }

    #[test]
    fn test_find_sum() {
        // Arrange
        let input = [10, 1, 2, 5, 7, 8, 10];

        // Act
        let result = find_partition_with_sum(&input, 8);

        // Assert
        assert_eq!(result[0], (1, 4));
        assert_eq!(result[1], (5, 6));
    }

}
