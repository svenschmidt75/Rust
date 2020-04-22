// Leetcode 56. Merge Intervals, https://leetcode.com/problems/merge-intervals/

fn merge_intervals(intervals: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // SS: sort at O(N log N)
    let mut sorted = intervals.to_owned();
    sorted.sort_by_key(|(min, max)| *min);

    let mut merged = vec![];

    let mut i = 0;
    while i < sorted.len() {
        let i1 = sorted[i];

        let mut j = i + 1;
        while j < sorted.len() {
            let mut i2 = sorted[j];
            if i2.0 <= i1.1 {
                j += 1;
                continue;
            }
            break;
        }

        let merged_start = i1.0;
        let merged_end = sorted[j - 1].1;
        merged.push((merged_start, merged_end));
        i = j;
    }

    merged
}

#[cfg(test)]
mod tests {
    use crate::merge_intervals;

    #[test]
    fn test1() {
        // Arrange
        let intervals = [(1, 3), (2, 6), (8, 10), (15, 18)];

        // Act
        let merged_intervals = merge_intervals(&intervals);

        // Assert
        assert_eq!(merged_intervals.len(), 3);
        assert_eq!(merged_intervals, vec![(1, 6), (8, 10), (15, 18)]);
    }

    #[test]
    fn test2() {
        // Arrange
        let intervals = [(1, 4), (4, 5)];

        // Act
        let merged_intervals = merge_intervals(&intervals);

        // Assert
        assert_eq!(merged_intervals.len(), 1);
        assert_eq!(merged_intervals, vec![(1, 5)]);
    }
}
