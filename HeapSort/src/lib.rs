struct MinHeap {
    heap: Vec<f64>,
}

impl MinHeap {
    fn new(data: &[f64]) -> MinHeap {
        let mut heap = MinHeap { heap: vec![] };
        for i in data {
            heap.insert(*i);
        }
        heap
    }

    fn insert(&mut self, v: f64) {
        // SS: add new element at the end of heap
        self.heap.push(v);
        self.up_heapify(self.heap.len() - 1);
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
            if let Some(left_child) = self.get_left_child(node_index) {
                if let Some(v) = self.find_internal(left_child, value) {
                    return Some(v);
                }
            };

            if let Some(right_child) = self.get_right_child(node_index) {
                if let Some(v) = self.find_internal(right_child, value) {
                    return Some(v);
                }
            };

            None
        }
    }

    fn delete(&mut self, node_index: usize) {
        // SS: swap node with last one
        self.heap[node_index] = self.heap[self.heap.len() - 1];

        // SS: delete element
        self.heap.pop();

        self.down_heapify(node_index);
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

        if let Some(left_child) = self.get_left_child(node_index) {
            if let Some(right_child) = self.get_right_child(node_index) {
                // SS: swap with smaller child, so the smallest element ends up at the top
                if self.heap[left_child] < self.heap[right_child] {
                    // SS: swap with left child
                    if self.heap[left_child] < self.heap[node_index] {
                        self.swap(left_child, node_index);
                        self.down_heapify(left_child);
                    }
                } else {
                    if self.heap[right_child] < self.heap[node_index] {
                        self.swap(right_child, node_index);
                        self.down_heapify(right_child);
                    }
                }
            } else {
                // SS: no right child, so swap with left
                if self.heap[left_child] < self.heap[node_index] {
                    self.swap(left_child, node_index);
                    self.down_heapify(left_child);
                }
            }
        };
    }

    fn get_parent(node_index: usize) -> usize {
        if node_index == 0 {
            panic!("root has no parent");
        }
        (node_index - 1) / 2
    }

    fn get_left_child(&self, node_index: usize) -> Option<usize> {
        let child_index = 2 * node_index + 1;
        if child_index < self.heap.len() {
            Some(child_index)
        } else {
            None
        }
    }

    fn get_right_child(&self, node_index: usize) -> Option<usize> {
        let child_index = 2 * (node_index + 1);
        if child_index < self.heap.len() {
            Some(child_index)
        } else {
            None
        }
    }

    fn is_minheap(&self) -> bool {
        self.is_minheap_internal(0)
    }

    fn is_minheap_internal(&self, node_index: usize) -> bool {
        // SS: traverse tree in level order
        if let Some(left_child) = self.get_left_child(node_index) {
            if left_child >= self.heap.len() {
                return true;
            }

            if self.heap[left_child] < self.heap[node_index] {
                return false;
            }

            if let Some(right_child) = self.get_right_child(node_index) {
                if right_child >= self.heap.len() {
                    return true;
                }

                if self.heap[right_child] < self.heap[node_index] {
                    return false;
                }

                return self.is_minheap_internal(left_child) && self.is_minheap_internal(right_child);
            };

            return self.is_minheap_internal(left_child);
        };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];

        // Act
        let heap = MinHeap::new(&data);

        // Assert
        assert!(heap.is_minheap())
    }

    #[test]
    fn test_find_1() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(3.0);

        // Assert
        assert!(found.is_some())
    }

    #[test]
    fn test_find_2() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(82.0);

        // Assert
        assert!(found.is_some())
    }

    #[test]
    fn test_find_3() {
        // Arrange
        let data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];
        let heap = MinHeap::new(&data);

        // Act
        let found = heap.find(92.0);

        // Assert
        assert!(found.is_none())
    }

    #[test]
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
