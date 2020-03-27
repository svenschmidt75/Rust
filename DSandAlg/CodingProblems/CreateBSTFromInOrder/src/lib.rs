// https://practice.geeksforgeeks.org/problems/array-to-bst/0

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

    fn preorder(&self, data: &mut Vec<i64>) {
        data.push(self.value);
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder(data);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder(data);
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

    fn preorder(&self) -> Vec<i64> {
        // SS: DF pre-order traversal
        if self.root.is_none() {
            vec![]
        } else {
            let mut result = vec![];
            self.root.as_ref().unwrap().preorder(&mut result);
            result
        }
    }
}

fn create_bst_from_inorder_internal(
    input: &[i64],
    min: usize,
    max: usize,
    bst: &mut BinarySearchTree,
) {
    if min < max {
        let mid = (min + max) / 2;
        let element = input[mid];
        bst.insert(element);
        create_bst_from_inorder_internal(input, min, mid, bst);
        create_bst_from_inorder_internal(input, mid + 1, max, bst);
    }
}

fn create_bst_from_inorder(input: &[i64]) -> BinarySearchTree {
    let mut bst = BinarySearchTree::new();
    create_bst_from_inorder_internal(input, 0, input.len(), &mut bst);
    bst
}

#[cfg(test)]
mod tests {
    use crate::create_bst_from_inorder;

    #[test]
    fn test() {
        // Arrange
        let input = [1, 2, 3, 4, 5, 6, 7];

        // Act
        let bst = create_bst_from_inorder(&input);
        let preorder = bst.preorder();

        // Assert
        assert_eq!(preorder, vec![4, 2, 1, 3, 6, 5, 7]);
    }
}
