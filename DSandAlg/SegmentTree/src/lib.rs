
// Segment Tree Range Minimum Query
// Tushar Roy - Coding Made Simple
// https://www.youtube.com/watch?v=ZBHKZF5w4YU&t=86s

use std::cmp;

struct MinSegmentTree {
    max: usize,
    data: Vec<i32>,
}

impl MinSegmentTree {
    fn new(array: &[i32]) -> MinSegmentTree {
        let n = (array.len() as f64).log2().ceil() as i32;
        let size = 2f64.powi(n + 1) as usize - 1;
        let mut segment_tree = MinSegmentTree {
            max: array.len(),
            data: vec![0; size],
        };
        MinSegmentTree::create_recursive(array, &mut segment_tree.data, 0, array.len() - 1, 0);
        segment_tree
    }

    fn create_recursive(
        array: &[i32],
        segment_tree_data: &mut [i32],
        min: usize,
        max: usize,
        pos: usize,
    ) -> i32 {
        // SS: base case
        if min == max {
            segment_tree_data[pos] = array[min];
            segment_tree_data[pos]
        } else {
            // SS: mid-point of interval
            let mid = (min + max) / 2;

            // SS: left subtree
            let left_min =
                MinSegmentTree::create_recursive(array, segment_tree_data, min, mid, 2 * pos + 1);

            // SS: right subtree
            let right_min = MinSegmentTree::create_recursive(
                array,
                segment_tree_data,
                mid + 1,
                max,
                2 * pos + 2,
            );

            let min = cmp::min(left_min, right_min);
            segment_tree_data[pos] = min;
            min
        }
    }

    fn find_min(&self, i1: usize, i2: usize) -> i32 {
        self.find_min_recursive(0, self.max - 1, i1, i2, 0)
    }

    fn find_min_recursive(&self, min: usize, max: usize, i1: usize, i2: usize, pos: usize) -> i32 {
        // SS: is query interval outside interval of current node?
        if max < i1 || i2 < min {
            // SS: no overlap
            i32::MAX
        } else if min >= i1 && max <= i2 {
            // SS: total overlap
            self.data[pos]
        } else {
            // SS: partial overlap

            // SS: mid-point of interval
            let mid = (min + max) / 2;

            let left_min = self.find_min_recursive(min, mid, i1, i2, 2 * pos + 1);
            let right_min = self.find_min_recursive(mid + 1, max, i1, i2, 2 * pos + 2);

            cmp::min(left_min, right_min)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MinSegmentTree;

    #[test]
    fn create_n4() {
        // Arrange
        let array = [-1, 0, 3, 6];

        // Act
        let segment_tree = MinSegmentTree::new(&array);

        // Assert
        assert_eq!(segment_tree.data, vec![-1, -1, 3, -1, 0, 3, 6]);
    }

    #[test]
    fn create_n6() {
        // Arrange
        let array = [-1, 3, 4, 0, 2, 1];

        // Act
        let segment_tree = MinSegmentTree::new(&array);

        // Assert
        assert_eq!(
            segment_tree.data,
            vec![-1, -1, 0, -1, 4, 0, 1, -1, 3, 0, 0, 0, 2, 0, 0]
        );
    }

    #[test]
    fn create_find_min_1() {
        // Arrange
        let array = [-1, 3, 4, 0, 2, 1];
        let segment_tree = MinSegmentTree::new(&array);

        // Act
        let min = segment_tree.find_min(2, 4);

        // Assert
        assert_eq!(min, 0);
    }

    #[test]
    fn create_find_min_2() {
        // Arrange
        let array = [-1, 3, 4, 0, 2, 1];
        let segment_tree = MinSegmentTree::new(&array);

        // Act
        let min = segment_tree.find_min(0, 4);

        // Assert
        assert_eq!(min, -1);
    }
}
