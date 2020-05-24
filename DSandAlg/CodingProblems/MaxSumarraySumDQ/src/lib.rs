// SS: Max Subarray Sum using Divide & Conquer, O(n log N.
// Can also use Kadane's algorithm, which uses the sliding
// window technique and is O(n).

fn solve(input: &[i32]) -> i32 {
    // SS: create prefix sum
    let difference = input
        .iter()
        .scan(input[0], |state, &b| {
            let diff = b - *state;
            *state = b;
            Some(diff)
        })
        .collect::<Vec<_>>();
    let (_, _, sum) = solve_recursive(&difference, 0, (difference.len() - 1) as i32);
    sum
}

fn solve_recursive(input: &[i32], min: i32, max: i32) -> (i32, i32, i32) {
    // SS: O(n log n)
    if min == max {
        (min, max, input[min as usize])
    } else {
        let mid = (min + max) / 2;

        let (lmin, lmax, lsum) = solve_recursive(input, min, mid);
        let (rmin, rmax, rsum) = solve_recursive(input, mid + 1, max);

        // SS: O(n)
        let (left_start, right_end, cross_sum) = find_cross_sum(input, min, mid, max);

        return if cross_sum >= lsum && cross_sum >= rsum {
            return (left_start, right_end, cross_sum);
        } else if lsum > rsum {
            (lmin, lmax, lsum)
        } else {
            (rmin, rmax, rsum)
        };
    }
}

fn find_cross_sum(input: &[i32], min: i32, mid: i32, max: i32) -> (i32, i32, i32) {
    // SS: find max sum across the mid-point, O(n)
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    let mut i = mid;
    let mut start_left = mid;
    while i >= min {
        let s = input[i as usize];
        sum += s;
        if sum > left_sum {
            left_sum = sum;
            start_left = i;
        }
        i -= 1;
    }

    let mut right_sum = i32::MIN;
    let mut sum = 0;
    let mut j = mid + 1;
    let mut start_right = mid;
    while j <= max {
        let s = input[j as usize];
        sum += s;
        if sum > right_sum {
            right_sum = sum;
            start_right = j;
        }
        j += 1;
    }

    (start_left, start_right, left_sum + right_sum)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        // Arrange
        let input = [
            100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97,
        ];

        // Act
        let sum = solve(&input);

        // Assert
        assert_eq!(sum, 43);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [63, 81, 101, 94, 106];

        // Act
        let sum = solve(&input);

        // Assert
        assert_eq!(sum, 43);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = [0, 5, 2, 14];

        // Act
        let sum = solve(&input);

        // Assert
        assert_eq!(sum, 14);
    }
}
