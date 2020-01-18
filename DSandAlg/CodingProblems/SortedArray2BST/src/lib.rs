use std::collections::VecDeque;

struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i64) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i64) {
        if value <= self.value {
            if self.left.is_some() {
                self.left.as_mut().unwrap().insert(value);
            } else {
                let node = Box::new(Node::new(value));
                self.left = Some(node);
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().insert(value);
            } else {
                let node = Box::new(Node::new(value));
                self.right = Some(node);
            }
        }
    }
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> BinarySearchTree {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: i64) {
        let node = Box::new(Node::new(value));
        match self.root {
            None => self.root = Some(node),
            Some(ref mut node) => node.insert(value),
        }
    }

    fn bfs_flatten(&self) -> Vec<i64> {
        if self.root.is_none() {
            vec![]
        } else {
            let mut values = vec![];
            let mut queue = VecDeque::new();
            queue.push_front(self.root.as_ref().unwrap());
            while queue.is_empty() == false {
                let current_node = queue.pop_back().unwrap();
                values.push(current_node.value);
                current_node
                    .left
                    .as_ref()
                    .map(|left_child| queue.push_front(left_child));
                current_node
                    .right
                    .as_ref()
                    .map(|right_child| queue.push_front(right_child));
            }
            values
        }
    }
}

fn construct_from_sorted_array(array: &[i64]) -> BinarySearchTree {
    let root = construct(array, 0, array.len());
    let mut bst = BinarySearchTree::new();
    bst.root = root;
    bst
}

fn construct(array: &[i64], min: usize, max: usize) -> Option<Box<Node>> {
    let parent_index = (min + max) / 2;
    let mut parent = Node::new(array[parent_index]);
    if min + 1 < max {
        parent.left = construct(array, min, parent_index);
        parent.right = construct(array, parent_index + 1, max);
    }
    Some(Box::new(parent))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bfs() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        bst.insert(9);
        bst.insert(4);
        bst.insert(20);
        bst.insert(1);
        bst.insert(6);
        bst.insert(15);
        bst.insert(170);

        // Assert
        let bfs_flattened = bst.bfs_flatten();
        assert_eq!(bfs_flattened, vec![9, 4, 20, 1, 6, 15, 170]);
    }

    #[test]
    fn create_from_sorted_array() {
        // Arrange
        let array = [1, 4, 6, 9, 15, 20, 170];

        // Act
        let bst = construct_from_sorted_array(&array);

        // Assert
        let bfs_flattened = bst.bfs_flatten();
        assert_eq!(bfs_flattened, vec![9, 4, 20, 1, 6, 15, 170]);
    }
}
