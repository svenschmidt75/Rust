// https://www.reddit.com/r/AlgoExpert/comments/f2bd9s/day_8_20200211_problem_of_the_day_asked_by_twitter/

fn add(input: &[i64], expected_sum: i64) -> Vec<Vec<i64>> {
    let mut low_index = 0;
    let mut high_index = input.len() - 1;

    let mut result = vec![];

    // SS: O(N) comparisons
    while low_index < high_index {
        let low_value = input[low_index];
        let high_value = input[high_index];
        let sum = low_value + high_value;
        if sum > expected_sum {
            high_index -= 1;
        } else if sum < expected_sum {
            low_index += 1;
        } else {
            result.push(vec![low_value, high_value]);
            low_index += 1;
            high_index -= 1;
        }
    }
    result
}

fn three_add_to_zero(input: &[i64]) -> Vec<Vec<i64>> {
    // SS: in total, O(N^2)

    // SS: sort at O(N log N)
    let mut sorted = input.to_vec();
    sorted.sort();

    let mut result = vec![];

    // SS: O(N) loop
    for i in 0..sorted.len() - 1 {
        // SS: a + b + c = 0, extract a, then find b + c = -a
        let a = sorted[i];
        let r = add(&sorted[i + 1..], -a);
        r.into_iter().for_each(|x| {
            let res = vec![a, x[0], x[1]];
            result.push(res);
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::three_add_to_zero;

    #[test]
    fn test1() {
        // Arrange
        let input = [0, -1, 2, -3, 1];

        // Act
        let result = three_add_to_zero(&input);

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [-3, 1, 2]);
        assert_eq!(result[1], [-1, 0, 1]);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [-1, 0, 1, 2, 3];

        // Act
        let result = three_add_to_zero(&input);

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], [-1, 0, 1]);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = [-3, -1, 0, 1, 2, 3, 4];

        // Act
        let result = three_add_to_zero(&input);

        // Assert
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], [-3, -1, 4]);
        assert_eq!(result[1], [-3, 0, 3]);
        assert_eq!(result[2], [-3, 1, 2]);
        assert_eq!(result[3], [-1, 0, 1]);
    }
}
