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


fn optimal_solution(input: &[u64], k: i64) -> u64 {
    // SS: O(N log N) solution utilizing binary search

    let partitions = find_partition_with_sum(input, 8);
    let valid = generate_and_validate(input, partitions[0], 8, k);

    println!("{:?}", valid);

    0

}

fn generate_and_validate(input: &[u64], partition: (usize, usize), sum: u64, k: i64) -> bool {
    // SS: we have two sets of numbers to partition,
    // 1. the set before the start of `partition`, and the set after.
    // Example: input = [10, 1, 2, 5, 7, 8, 10], partition: (1, 4], sum=8, then
    // the two sets are [0, 1] and (4, 7).
    // These two sets have to form k partitions in total...
    let mut remaining_partitions = k - 1;
    if partition.1 < input.len() {
        // SS: leave one for the end
        remaining_partitions -= 1;
    }

    let mut idx = 0;
    while remaining_partitions > 0 {
        let mut s = 0;
        while idx < partition.0 && s < sum {
            s += input[idx];
            idx += 1;
        }

        if idx == partition.0 {
            if s < sum {
                // SS: the partition starting before `partition` has sum smaller than `sum`,
                // so no solution
                return false;
            }

            // SS: cannot generate any more partitions
            break;
        } else if s > sum {
            remaining_partitions -= 1;
            idx -= 1;
        }
    }

    idx = partition.1;
    while remaining_partitions > 0 {
        let mut s = 0;
        while idx < input.len() && s < sum {
            s += input[idx];
            idx += 1;
        }

        if idx == input.len() {
            if s < sum {
                // SS: the partition starting after `partition` has sum smaller than `sum`,
                // so no solution
                return false;
            }

            // SS: cannot generate any more partitions
            break;
        } else if s > sum {
            remaining_partitions -= 1;
            idx -= 1;
        }
    }

    true
}

fn find_partition_with_sum(input: &[u64], sum: u64) -> Vec<(usize, usize)> {
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
    use crate::{brute_force, find_partition_with_sum, generate_and_validate};

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

    #[test]
    fn test_validate_partition() {
        // Arrange
        let input = [10, 1, 2, 5, 7, 8, 10];

        // Act
        let is_valid = generate_and_validate(&input, (1, 4), 8, 3);

        // Assert
        assert_eq!(is_valid, true);
    }

}
