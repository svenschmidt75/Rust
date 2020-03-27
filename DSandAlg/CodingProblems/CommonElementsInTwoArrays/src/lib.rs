// Cracking the Coding Interview
// 6th ed, page 73

use std::collections::HashSet;

fn find_common_1(a: &[u32], b: &[u32]) -> Vec<(usize, usize)> {
    // SS: Find common elements in arrays a and b, both of which
    // are assumed sorted.
    let mut result = vec![];

    // SS: runtime is O(min(a, b))
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        let e1 = a[i];
        let e2 = b[j];
        if e1 < e2 {
            i += 1;
        } else if e1 > e2 {
            j += 1;
        } else {
            result.push((i, j));
            i += 1;
            j += 1;
        }
    }

    result
}

fn find_common_2(a: &[u32], b: &[u32]) -> Vec<usize> {
    // SS: Find common elements in arrays a and b, both of which
    // are assumed sorted.
    let mut result = vec![];

    // SS: O(a) memory
    let mut hash = HashSet::new();
    a.iter().for_each(|e| {
        hash.insert(e);
    });

    // SS: runtime is O(b)
    for j in 0..b.len() {
        let e = b[j];
        if hash.contains(&e) {
            result.push(j);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1()
    {
        // Arrange
        let a = [13, 27, 35, 40, 49, 55, 59];
        let b = [17, 35, 39, 40, 55, 58, 60];

        // Act
        let result = find_common_1(&a, &b);

        // Assert
        assert_eq!(result, vec![(2, 1), (3, 3), (5, 4)]);
    }

    #[test]
    fn test2()
    {
        // Arrange
        let a = [13, 27, 35, 40, 49, 55, 59];
        let b = [17, 35, 39, 40, 55, 58, 60];

        // Act
        let result = find_common_2(&a, &b);

        // Assert
        assert_eq!(result, vec![1, 3, 4]);
    }
}
