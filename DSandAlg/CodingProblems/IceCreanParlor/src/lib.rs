// https://www.youtube.com/watch?v=Ifwf3DBN1sc&list=PLI1t_8YX-ApvMthLj56t1Rf-Buio5Y8KL&index=2

fn ice_cream_parlor(cost: &[u32], m: u32) -> (usize, usize) {
    // SS: Given the cost per ice cream scoop, and an amount m,
    // find two flavors that sum up to m.

    // Could also use a hash map...

    // SS: sort with original index, O(N log N)
    let mut sorted = cost.iter().enumerate().collect::<Vec<_>>();
    sorted.sort_by_key(|&(idx, value)| *value);

    // SS: this loop is doing a search of O(log N) for up to each ice cream scoop, so O(N log N)
    // total runtime...
    for i in 0..cost.len() {
        let (index1, &cost1) = sorted[i];

        if cost1 >= m {
            return (sorted.len(), sorted.len());
        }

        let cost2 = m - cost1;

        // SS: imagine this is Binary Search, i.e. O(log N), since `sorted` is sorted...
        if let Some((idx2, _)) = sorted[i + 1..].iter().find(|(idx2, &c2)| c2 == cost2) {
            return (i, *idx2);
        };
    }

    (cost.len(), cost.len())
}

#[cfg(test)]
mod tests {
    use crate::ice_cream_parlor;

    #[test]
    fn test1() {
        // Arrange
        let ice_cream_cost = [1, 4, 5, 3, 2];

        // Act
        let (idx1, idx2) = ice_cream_parlor(&ice_cream_cost, 4);

        // Assert
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let ice_cream_cost = [2, 2, 4, 3];

        // Act
        let (idx1, idx2) = ice_cream_parlor(&ice_cream_cost, 4);

        // Assert
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
    }
}
