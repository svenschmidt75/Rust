// https://www.reddit.com/r/AlgoExpert/comments/euowcd/day_5_20200127_problem_of_the_day_asked_by_amazon/
// Alternative solutions:
// 1. Create character frequency, no sorting necessary.
// Then, from high to low frequency, combine...

// 2. Create max priority queue, character frequency = priority
// pop off, until no element left in queue, or only one with one character

use std::cmp;

fn rearrange_characters(input: &str) -> String {
    // SS: check for empty

    let mut result: Vec<char> = vec![];

    // SS: sort input, O(N log N)
    let mut input_copy = input.clone().chars().into_iter().collect::<Vec<_>>();
    input_copy.sort();

    // generate ranges
    // runtime O(N), space O(N)
    let mut ranges = vec![];
    let mut range = (input_copy[0], 0, 0);
    let mut in_range = false;
    for i in 0..input_copy.len() {
        let c = input_copy[i];
        if in_range {
            if c == range.0 {
                range.2 = i;
            } else {
                ranges.push(range);
                range = (c, i, i);
            }
        } else {
            in_range = true;
            range = (c, i, i);
        }
    }

    if in_range {
        ranges.push(range);
    }

    if ranges.len() == 1 && ranges[0].1 == ranges[0].2 {
        // SS: there is only one range, with one character
        result.push(ranges[0].0);
    } else {
        // SS: all of the following loops at most N,
        // although there is a nested loop! It still is O(N)...

        // SS: more than one range
        let mut r1_idx = 0;
        let mut idx1 = 0;

        let mut r2_idx = 1;
        let mut idx2 = 0;

        while r1_idx < ranges.len() && r2_idx < ranges.len() {
            let mut r1 = ranges[r1_idx];
            let mut r2 = ranges[r2_idx];

            while idx1 <= (r1.2 - r1.1) && idx2 <= (r2.2 - r2.1) {
                result.push(r1.0);
                idx1 += 1;

                result.push(r2.0);
                idx2 += 1;
            }

            if idx1 == r1.2 - r1.1 + 1 {
                // SS: new r1 range
                r1_idx = cmp::max(r1_idx, r2_idx) + 1;
                idx1 = 0;
            }

            if idx2 == r2.2 - r2.1 + 1 {
                // SS: new r2 range
                r2_idx = cmp::max(r1_idx, r2_idx) + 1;
                idx2 = 0;
            }
        }

        // SS: check remaining
        if r1_idx < ranges.len() {
            // done, valid?
            if r2_idx >= ranges.len() {
                if ranges[r1_idx].2 - ranges[r1_idx].1 - idx1 == 0 {
                    result.push(ranges[r1_idx].0);
                } else {
                    // not valid
                    result.clear();
                }
            } else {
                // unbalanced, not valid
                result.clear();
            }
        } else if r2_idx < ranges.len() {
            // done, valid?
            if r1_idx >= ranges.len() {
                if ranges[r2_idx].2 - ranges[r2_idx].1 - idx2 == 0 {
                    result.push(ranges[r2_idx].0);
                } else {
                    // not valid
                    result.clear();
                }
            } else {
                // unbalanced, not valid
                result.clear();
            }
        } else {
            // not valid
            result.clear();
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::rearrange_characters;

    #[test]
    fn test1() {
        // Arrange
        let input = "aabbc";

        // Act
        let result = rearrange_characters(&input);

        // Assert
        assert_eq!(result, "ababc");
    }

    #[test]
    fn test2() {
        // Arrange
        let input = "aaaabbc";

        // Act
        let result = rearrange_characters(&input);

        // Assert
        assert_eq!(result, "ababaca");
    }

    #[test]
    fn test3() {
        // Arrange
        let input = "aaab";

        // Act
        let result = rearrange_characters(&input);

        // Assert
        assert_eq!(result, "");
    }
}
