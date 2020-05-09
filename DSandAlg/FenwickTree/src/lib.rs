// https://www.udemy.com/course/introduction-to-data-structures/learn/lecture/7261064#overview

struct FenwickTree {
    data: Vec<i32>,
}

impl FenwickTree {
    fn lsb(v: i32) -> i32 {
        v & -v
    }

    fn new(array: &[i32]) -> FenwickTree {
        // SS: construct Fenwick tree

        // SS: make 1-based
        let mut data = vec![0; array.len() + 1];
        data[1..].clone_from_slice(array);

        for i in 1..data.len() {
            let parent_index = i + FenwickTree::lsb(i as i32) as usize;
            if parent_index < data.len() {
                data[parent_index] += data[i];
            }
        }

        FenwickTree { data }
    }

    fn add(&mut self, index: usize, v: i32) {
        let mut j = index;
        while j <= self.data.len() {
            self.data[j] += v;
            j = j + FenwickTree::lsb(j as i32) as usize;
        }
    }

    fn prefix_sum(&self, index: usize) -> i32 {
        let mut ps = 0;
        let mut j = index;
        while j > 0 {
            ps += self.data[j];
            j = j - FenwickTree::lsb(j as i32) as usize;
        }
        ps
    }

    fn range_query(&self, index1: usize, index2: usize) -> i32 {
        let ps1 = self.prefix_sum(index1);
        let ps2 = self.prefix_sum(index2 - 1);
        let ps = ps2 - ps1;
        ps
    }
}

#[cfg(test)]
mod tests {
    use crate::FenwickTree;

    #[test]
    fn prefix_sum() {
        // Arrange
        let data = [3, 4, -2, 7, 3, 11, 5, -8, -9, 2, 4, -8];
        let fenwick = FenwickTree::new(&data);

        // Act
        let ps = fenwick.prefix_sum(12);

        // Assert
        let ps2 = data.iter().fold(0, |state, x| state + *x);
        assert_eq!(ps, ps2);
    }

    #[test]
    fn range_query() {
        // Arrange
        let data = [3, 4, -2, 7, 3, 11, 5, -8, -9, 2, 4, -8];
        let fenwick = FenwickTree::new(&data);

        // Act
        let range = fenwick.range_query(4, 8);

        // Assert
        let ps1 = fenwick.prefix_sum(4);
        let ps2 = fenwick.prefix_sum(7);
        assert_eq!(range, ps2 - ps1);
    }

    #[test]
    fn point_update() {
        // Arrange
        let data = [3, 4, -2, 7, 3, 11, 5, -8, -9, 2, 4, -8];
        let mut fenwick = FenwickTree::new(&data);

        // Act
        fenwick.add(5, 1);

        // Assert
        let range = fenwick.range_query(4, 8);
        assert_eq!(range, 20);
    }
}
