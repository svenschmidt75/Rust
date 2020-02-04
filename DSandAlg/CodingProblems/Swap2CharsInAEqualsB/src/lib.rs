// https://www.reddit.com/r/AlgoExpert/comments/evoh1a/day_6_20200129_problem_of_the_day_asked_by_airbnb/

fn swap(input: &mut [char], a: usize, b: usize) {
    let tmp = input[a];
    input[a] = input[b];
    input[b] = tmp;
}

fn solve1(a: &str, b: &str) -> bool {
    // SS: runtime complexity: O(k1 * k2), where k1 = length(a), k2=length(b)
    if a.len() < 2 || a.len() != b.len() {
        false
    } else {
        let mut avec = a.chars().into_iter().collect::<Vec<_>>();
        let bvec = b.chars().into_iter().collect::<Vec<_>>();

        for i in 0..=(a.len() - 2) {
            swap(&mut avec, i, i + 1);
            if avec == bvec {
                return true;
            }

            // SS: restore
            swap(&mut avec, i, i + 1);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn test1() {
        // Arrange
        let a = "ab";
        let b = "ba";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let a = "ab";
        let b = "ab";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        // Arrange
        let a = "aa";
        let b = "aa";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        // Arrange
        let a = "aaaaaaabc";
        let b = "aaaaaaacb";

        // Act
        let result = solve1(a, b);

        // Assert
        assert_eq!(result, true);
    }
}
