use std::cmp;
use std::cmp::{Ordering, Reverse};

// https://www.reddit.com/r/AlgoExpert/comments/esbsp6/day_2_20200122_problem_of_the_day_asked_by/

fn three_term_product(array: &[i64]) -> Vec<i64> {
    let mut solutions = vec![];
    if array.len() < 3 {
        solutions
    } else {
        let mut largest_product = std::i64::MIN;
        let mut best_a1 = 0;
        let mut best_a2 = 0;
        let mut best_a3 = 0;

        // SS: O(N^3) solution
        for i in 0..array.len() {
            let a1 = array[i];
            for j in (i + 1)..array.len() {
                let a2 = array[j];
                for k in (j + 1)..array.len() {
                    let a3 = array[k];
                    let prod = a1 * a2 * a3;
                    if prod > largest_product {
                        best_a1 = a1;
                        best_a2 = a2;
                        best_a3 = a3;
                        largest_product = prod;
                    }
                }
            }
        }
        solutions.push(best_a1);
        solutions.push(best_a2);
        solutions.push(best_a3);
        solutions
    }
}

fn three_term_product_rec(array: &[i64]) -> i64 {
    if array.len() < 3 {
        0
    } else {
        // SS: O(2^N) solution
        three_term_product_rec_internal(&array, array.len(), &[])
    }
}

fn three_term_product_rec_internal(array: &[i64], n: usize, bucket: &[i64]) -> i64 {
    if n == 0 {
        if bucket.len() == 3 {
            bucket.iter().product::<i64>()
        } else {
            0
        }
    } else {
        // SS: could check for size of bucket to avoid going deeper than 3

        let array_position = array.len() - n;
        let best_product_left = three_term_product_rec_internal(&array, n - 1, bucket);

        let mut new_right_bucket = bucket.to_vec();
        new_right_bucket.push(array[array_position]);
        let best_product_right =
            three_term_product_rec_internal(&array, n - 1, &new_right_bucket[..]);

        cmp::max(best_product_left, best_product_right)
    }
}

fn three_term_product_sort(array: &[i64]) -> i64 {
    if array.len() < 3 {
        0
    } else {
        let mut a = array.to_vec();

        // SS: O(N log N) runtime
        a.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // SS: if the 1st element is negative, multiply it with the second, so
        // it becomes positive, then with the largest, which must be the last.
        // It cannot be a[2], as if a[2] < 0, we end up with a negative number...

        let p1 = a[0] * a[1] * a[array.len() - 1];
        let p2 = a[array.len() - 3] * a[array.len() - 2] * a[array.len() - 1];
        cmp::max(p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let array = [-4, -4, 2, 8];

        // Act
        let solution = three_term_product(&array);

        // Assert
        assert_eq!(solution.iter().product::<i64>(), 128);
    }

    #[test]
    fn test12() {
        // Arrange
        let array = [-4, -4, 2, 8];

        // Act
        let solution = three_term_product_rec(&array);

        // Assert
        assert_eq!(solution, 128);
    }

    #[test]
    fn test13() {
        // Arrange
        let array = [-4, -4, 2, 8];

        // Act
        let solution = three_term_product_sort(&array);

        // Assert
        assert_eq!(solution, 128);
    }

    #[test]
    fn test21() {
        // Arrange
        let array = [-8, 4, 2, 8];

        // Act
        let solution = three_term_product(&array);

        // Assert
        assert_eq!(solution.iter().product::<i64>(), 64);
    }

    #[test]
    fn test22() {
        // Arrange
        let array = [-8, 4, 2, 8];

        // Act
        let solution = three_term_product_rec(&array);

        // Assert
        assert_eq!(solution, 64);
    }

    #[test]
    fn test23() {
        // Arrange
        let array = [-8, 4, 2, 8];

        // Act
        let solution = three_term_product_sort(&array);

        // Assert
        assert_eq!(solution, 64);
    }

    #[test]
    fn test31() {
        // Arrange
        let array = [-4, -5, -1, 3, 0, -2, 45, 1];

        // Act
        let solution = three_term_product(&array);

        // Assert
        assert_eq!(solution.iter().product::<i64>(), 900);
    }

    #[test]
    fn test32() {
        // Arrange
        let array = [-4, -5, -1, 3, 0, -2, 45, 1];

        // Act
        let solution = three_term_product_rec(&array);

        // Assert
        assert_eq!(solution, 900);
    }

    #[test]
    fn test33() {
        // Arrange
        let array = [-4, -5, -1, 3, 0, -2, 45, 1];

        // Act
        let solution = three_term_product_sort(&array);

        // Assert
        assert_eq!(solution, 900);
    }
}
