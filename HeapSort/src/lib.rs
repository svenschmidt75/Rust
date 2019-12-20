
struct MinHeap {
    heap: Vec<f64>
}

impl MinHeap {

    fn new(data: &[f64]) -> MinHeap {
        let mut heap = MinHeap {heap: vec![]};
        for i in data {
            heap.insert(*i);
        }
        heap
    }

    fn insert(&mut self, v: f64) {
        self.heap.push(v);
        self.up_heapify(self.heap.len() - 1);
    }

    fn up_heapify(&mut self, node_index: usize) {
        if node_index == 0 {
            // done
            return;
        }

        let parent_node = MinHeap::get_parent(node_index);
        if self.heap[parent_node] > self.heap[node_index] {
            self.swap(parent_node, node_index);
            self.up_heapify(parent_node);
        }
    }

    fn down_heapify(&mut self, node_index: usize) {
        if node_index >= self.heap.len() {
            // done
            return;
        }

        let left_child = MinHeap::get_left_child(node_index);
        if self.heap[left_child] < self.heap[node_index] {
            self.swap(left_child, node_index);
            self.down_heapify(left_child);
        }

        let right_child = MinHeap::get_right_child(node_index);
        if self.heap[right_child] < self.heap[node_index] {
            self.swap(right_child, node_index);
            self.down_heapify(right_child);
        }
    }

    fn find(&self, value: f64) -> Option<usize> {
        self.find_internal(0, value)
    }

    fn find_internal(&self, node_index: usize, value: f64) -> Option<usize> {
        // SS: pre-order traversal
        if node_index >= self.heap.len() {
            return None;
        }
        if self.heap[node_index] == value {
            Some(node_index)
        } else {
            // SS: min heap, so if smaller than current node, cannot be in tree
            if value < self.heap[node_index] {
                return None;
            }
            let left_child = MinHeap::get_left_child(node_index);
            if let Some(v) = self.find_internal(left_child, value) {
                return Some(v);
            }
            let right_child = MinHeap::get_right_child(node_index);
            if let Some(v) = self.find_internal(right_child, value) {
                return Some(v);
            }
            None
        }
    }

    fn delete(&mut self, node_index: usize) {
        self.delete_internal(node_index);
        self.down_heapify(node_index);
    }

    fn delete_internal(&mut self, node_index: usize) {
        if node_index >= self.heap.len() {
            panic!("Node does not exist");
        }
        if node_index == self.heap.len() -1 {
            // SS: last element, remove
            self.heap.pop();
        } else {
            let right_child = MinHeap::get_right_child(node_index);
            if right_child >= self.heap.len() {
                return;
            } else {
                // SS: swap
                self.swap(node_index, right_child);
                self.delete_internal(right_child);
                return;
            }

            let left_child = MinHeap::get_left_child(node_index);
            if left_child >= self.heap.len() {
                return;
            } else {
                // SS: swap
                self.swap(node_index, left_child);
                self.delete_internal(left_child);
                return;
            }
        }
    }

    fn swap(&mut self, n1: usize, n2: usize) {
        if n1 >= self.heap.len() {
            panic!("Node does not exist");
        }
        if n2 >= self.heap.len() {
            panic!("Node does not exist");
        }
        let tmp = self.heap[n1];
        self.heap[n1] = self.heap[n2];
        self.heap[n2] = tmp;
    }

    fn get_parent(node_index: usize) -> usize {
        if node_index == 0 {
            panic!("root has no parent");
        }
        (node_index - 1) / 2
    }

    fn get_left_child(node_index: usize) -> usize {
        2 * node_index + 1
    }

    fn get_right_child(node_index: usize) -> usize {
        2 * (node_index + 1)
    }

    fn is_minheap(&self) -> bool {
        self.is_minheap_internal(0)
    }

    fn is_minheap_internal(&self, node_index: usize) -> bool {
        // SS: traverse tree in level order
        let left_child = MinHeap::get_left_child(node_index);
        if left_child >= self.heap.len() {
            return true;
        }
        if self.heap[left_child] < self.heap[node_index] {
            return false;
        }

        let right_child = MinHeap::get_right_child(node_index);
        if right_child >= self.heap.len() {
            return true;
        }
        if self.heap[right_child] < self.heap[node_index] {
            return false;
        }

        self.is_minheap_internal(left_child) && self.is_minheap_internal(right_child)
    }
}


#[cfg(test)]
mod tests {
    use crate::MinHeap;

    # [test]
    fn test_create() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];

        // Act
        let heap = MinHeap::new(&data);

        // Assert
        assert!(heap.is_minheap())
    }

    # [test]
    fn test_find_1() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(3.0);

        // Assert
        assert!(found.is_some())
    }

    # [test]
    fn test_find_2() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(82.0);

        // Assert
        assert!(found.is_some())
    }

    # [test]
    fn test_find_3() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(92.0);

        // Assert
        assert!(found.is_none())
    }

    # [test]
    fn test_delete_root() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let mut heap = MinHeap::new(&data);

        // Act
        heap.delete(0);

        // Assert
        assert!(heap.is_minheap())
    }

}