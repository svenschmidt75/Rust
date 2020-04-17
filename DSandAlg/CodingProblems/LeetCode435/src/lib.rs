// LeetCode 435, https://leetcode.com/problems/non-overlapping-intervals/

use std::cmp;

fn min_intervals_to_remove(intervals: &[(usize, usize)]) -> usize {
    // SS: sort at O(N log N)
    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|&key| key.0);

    let mut removed_count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < sorted.len() - 1 {
        let pair1 = sorted[i];
        j += 1;
        while j < sorted.len() {
            let pair2 = sorted[j];

            // SS: check for overlap
            if pair2.0 < pair1.1 && pair1.0 < pair2.1 {
                if pair1.1 < pair2.1 {
                    // SS: keep interval i
                    j += 1;
                } else {
                    // SS: keep interval j
                    i = j;
                    j += 1;
                }
                removed_count += 1;
            } else if pair1.0 == pair1.0 && pair1.1 == pair2.1 {
                // SS: both interval are the same
                i = j;
                j += 1;
                removed_count += 1;
            } else {
                // SS: no overlap
                i = j;
                break;
            }
        }

        if j == sorted.len() {
            break;
        }
    }

    removed_count
}

#[cfg(test)]
mod tests {
    use crate::min_intervals_to_remove;

    #[test]
    fn test1() {
        // Arrange
        let intervals = [(1, 2), (2, 3), (3, 4), (1, 3)];

        // Act
        let min_to_remove = min_intervals_to_remove(&intervals);

        // Assert
        assert_eq!(min_to_remove, 1);
    }

    #[test]
    fn test2() {
        // Arrange
        let intervals = [(1, 2), (1, 2), (1, 2)];

        // Act
        let min_to_remove = min_intervals_to_remove(&intervals);

        // Assert
        assert_eq!(min_to_remove, 2);
    }

    #[test]
    fn test3() {
        // Arrange
        let intervals = [(1, 2), (2, 3)];

        // Act
        let min_to_remove = min_intervals_to_remove(&intervals);

        // Assert
        assert_eq!(min_to_remove, 0);
    }

    #[test]
    fn test4() {
        // Arrange
        let intervals = [(1, 5), (2, 4), (3, 6), (8, 10)];

        // Act
        let min_to_remove = min_intervals_to_remove(&intervals);

        // Assert
        assert_eq!(min_to_remove, 2);
    }
}
