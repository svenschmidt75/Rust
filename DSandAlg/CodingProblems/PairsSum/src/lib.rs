
/* Given a sorted array of numbers and a number, find a pair
 * s.t. the sum of both equals the sum.
*/

fn sum_pairs(array: &[u64], sum: u64) -> Vec<(usize, usize)> {
    let mut v = vec![];

    let mut min = 0;
    let mut max = array.len() - 1;

    while min < max {
        // SS: find lower bound
        while min < max && array[min] + array[max] < sum {
            min = min + 1;
        }

        // SS: find upper bound
        while min < max && array[min] + array[max] > sum {
            max = max - 1;
        }

        if array[min] + array[max] == sum {
            v.push((min, max));
            min = min + 1;
            max = max - 1;
        }
    }

    v
}


#[cfg(test)]
mod tests {
    use crate::sum_pairs;

    #[test]
    fn test1() {
        // Arrange
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Act
        let pairs = sum_pairs(&array, 8);

        // Assert
        assert_eq!(3, pairs.len());
    }

    #[test]
    fn test2() {
        // Arrange
        let array = [1, 3, 4, 5, 6, 7, 8, 9];

        // Act
        let pairs = sum_pairs(&array, 8);

        // Assert
        assert_eq!(2, pairs.len());
    }

}
