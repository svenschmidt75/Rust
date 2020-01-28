// https://www.reddit.com/r/AlgoExpert/comments/etbi4o/day_4_20200124_problem_of_the_day_asked_by/

use std::cmp;

// SS: O(N log N)
fn merge_overlapping_ranges_1(ranges: &mut [(i64, i64)]) -> Vec<(i64, i64)> {
    // SS: check for empty
    if ranges.len() <= 1 {
        ranges.to_vec()
    } else {
        // SS: O(N log N) to sort
        ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut result = vec![];
        let mut current_range = ranges[0];

        // SS: O(N)
        for i in 1..ranges.len() {
            let r = ranges[i];
            if r.0 <= current_range.1 {
                // SS: overlap
                current_range.1 = cmp::max(current_range.1, r.1);
            } else {
                // SS: no overlap
                result.push(current_range);
                current_range = r;
            }
        }

        result.push(current_range);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_overlapping_ranges_1;

    #[test]
    fn test1() {
        // Arrange
        let mut input = [(1, 3), (4, 10), (5, 8), (20, 25)];

        // Act
        let result = merge_overlapping_ranges_1(&mut input);

        // Assert
        assert_eq!(&result[..], &[(1, 3), (4, 10), (20, 25)]);
    }

    #[test]
    fn test2() {
        // Arrange
        let mut input = [(1, 3), (4, 10), (2, 12)];

        // Act
        let result = merge_overlapping_ranges_1(&mut input);

        // Assert
        assert_eq!(&result[..], &[(1, 12)]);
    }

    #[test]
    fn test3() {
        // Arrange
        let mut input = [(1, 3), (4, 6), (2, 12), (20, 25)];

        // Act
        let result = merge_overlapping_ranges_1(&mut input);

        // Assert
        assert_eq!(&result[..], &[(1, 12), (20, 25)]);
    }

}
