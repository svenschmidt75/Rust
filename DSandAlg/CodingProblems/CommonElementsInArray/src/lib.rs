use std::collections::HashSet;


/* Return true if two arrays have a common element, else false.
 * Do not assume any sorting of elements in the arrays.
 */

fn elements_in_common(array1: &[char], array2: &[char]) -> bool {
    /* A naive solution is to check each element from array1 against each element from
     * array2, which has O(n*m) time complexity and O(1) space complexity.
     *
     * If we store the elements for array1 in a hash set at O(n) additional space
     * complexity, we can achieve O(n + m) runtime, O(n) for each insert of array1 into
     * the hash set and O(m) for each check against elements in array2.
    */
    let mut set = HashSet::new();

    // SS: insert each element from array1 into hash set
    // O(n) space complexity and O(n) time complexity, as insert into hash set is O(1)
    for &elem in array1 {
        set.insert(elem);
    }

    // SS: O(1) lookup M times, so O(m) time complexity
    for elem in array2 {
        if set.contains(elem) {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use crate::elements_in_common;

    #[test]
    fn example1() {
        // Assert
        let array1 = ['a', 'b', 'c', 'x'];
        let array2 = ['z', 'y', 'i'];

        // Act
        let result = elements_in_common(&array1, &array2);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn example2() {
        // Assert
        let array1 = ['a', 'b', 'c', 'x'];
        let array2 = ['z', 'y', 'x'];

        // Act
        let result = elements_in_common(&array1, &array2);

        // Assert
        assert_eq!(true, result);
    }

}
