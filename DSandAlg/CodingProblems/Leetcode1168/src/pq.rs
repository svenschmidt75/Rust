use std::{cmp, mem};

struct MinHeap<K, V>
where
    K: Eq + PartialOrd + Copy,
    V: Copy,
{
    heap: Vec<(K, V)>,
}

impl<K, V> MinHeap<K, V>
where
    K: Eq + PartialOrd + Copy,
    V: Copy,
{
    fn new_from_data(data: &[(K, V)]) -> MinHeap<K, V> {
        let mut heap = MinHeap {
            heap: Vec::<(K, V)>::new(),
        };
        for &i in data {
            heap.insert(i);
        }
        heap
    }

    fn new() -> MinHeap<K, V> {
        MinHeap {
            heap: Vec::<(K, V)>::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn insert(&mut self, v: (K, V)) {
        // SS: add new element at the end of heap
        self.heap.push(v);
        self.up_heapify(self.heap.len() - 1);
    }

    fn find(&self, value: &(K, V)) -> Option<usize> {
        self.find_internal(0, value)
    }

    fn find_internal(&self, node_index: usize, value: &(K, V)) -> Option<usize> {
        // SS: pre-order traversal
        if node_index >= self.heap.len() {
            return None;
        }
        if self.heap[node_index].0 == value.0 {
            Some(node_index)
        } else {
            // SS: min heap, so if smaller than current node, cannot be in tree
            if value.0 < self.heap[node_index].0 {
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

    fn delete(&mut self, node_index: usize) -> Option<(K, V)> {
        if self.heap.len() > 1 {
            // SS: swap node with last one
            let length = self.heap.len() - 1;
            let (a, b) = self.heap.split_at_mut(length);
            mem::swap(&mut a[node_index], &mut b[0]);
        }

        // SS: delete element
        let element = self.heap.pop();

        self.down_heapify(node_index);

        element
    }

    fn swap(&mut self, n1: usize, n2: usize) {
        if n1 >= self.heap.len() {
            panic!("Node does not exist");
        }
        if n2 >= self.heap.len() {
            panic!("Node does not exist");
        }

        let mut nlow = cmp::min(n1, n2);
        let mut nhigh = cmp::max(n1, n2);
        let (left, right) = self.heap.split_at_mut(nhigh);
        mem::swap(&mut left[nlow], &mut right[0]);
    }

    fn up_heapify(&mut self, node_index: usize) {
        if node_index == 0 {
            // done
            return;
        }

        let mut parent_node = MinHeap::<K, V>::get_parent(node_index);
        if self.heap[parent_node].0 < self.heap[node_index].0 {
            self.swap(parent_node, node_index);

            // SS: heapify all the way to the top
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
                if self.heap[left_child].0 < self.heap[right_child].0 {
                    // SS: swap with left child
                    if self.heap[left_child].0 < self.heap[node_index].0 {
                        self.swap(left_child, node_index);
                        self.down_heapify(left_child);
                    }
                } else {
                    if self.heap[right_child].0 < self.heap[node_index].0 {
                        self.swap(right_child, node_index);
                        self.down_heapify(right_child);
                    }
                }
            } else {
                // SS: no right child, so swap with left
                if self.heap[left_child].0 < self.heap[node_index].0 {
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
}

pub struct PriorityQueue<T>
where
    T: Copy,
{
    heap: MinHeap<i64, T>,
}

impl<T> PriorityQueue<T>
where
    T: Copy,
{
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            heap: MinHeap::new(),
        }
    }

    pub fn insert(&mut self, priority: i64, value: T) {
        self.heap.insert((priority, value));
    }

    pub fn pop(&mut self) -> (i64, T) {
        self.heap.delete(0).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
