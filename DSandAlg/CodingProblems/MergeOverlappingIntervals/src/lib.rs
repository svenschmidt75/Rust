// 56. Merge Intervals
// https://leetcode.com/problems/merge-intervals/

fn solve(input: &[(u16, u16)]) -> Vec<(u16, u16)> {
    // SS: runtime is O(n log n)

    if input.len() < 2 {
        input.to_owned()
    } else {
        // SS: sort intervals w.r.t. start value, O(n log n)
        let mut intervals = input.to_owned();
        intervals.sort_by_key(|(min, max)| *min);

        let mut result = vec![];

        // SS: merge intervals, O(n)
        let mut i1 = intervals[0];
        let mut i = 1;
        while i < input.len() {
            let i2 = intervals[i];
            if i1.1 >= i2.0 {
                i1 = (i1.0, i2.1);
            } else {
                // SS: no overlap
                result.push(i1);
                i1 = i2;
            }
            i += 1;
        }
        result.push(i1);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        // Arrange
        let input = [(1, 3), (2, 6), (8, 10), (15, 18)];

        // Act
        let result = solve(&input);

        // Assert
        assert_eq!(result, vec![(1, 6), (8, 10), (15, 18)]);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [(1, 4), (4, 5)];

        // Act
        let result = solve(&input);

        // Assert
        assert_eq!(result, vec![(1, 5)]);
    }
}
