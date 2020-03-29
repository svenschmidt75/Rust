// Cracking the Coding Interview
// 6th ed, p. 109, ex. 4.2

use std::cmp;
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
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> BinarySearchTree {
        BinarySearchTree { root: None }
    }

    fn create_from_sorted_array(input: &[i64]) -> BinarySearchTree {
        // SS: create a BST from a sorted array (increasing order)
        let mut bst = BinarySearchTree::new();
        let mid = input.len() / 2;
        let value = input[mid];
        let mut root = Box::new(Node::new(value));
        root.left = BinarySearchTree::create_from_sorted_array_internal(&mut root, input, 0, mid);
        root.right = BinarySearchTree::create_from_sorted_array_internal(
            &mut root,
            input,
            mid + 1,
            input.len(),
        );
        bst.root = Some(root);
        bst
    }

    fn create_from_sorted_array_internal(
        parent: &mut Node,
        input: &[i64],
        min: usize,
        max: usize,
    ) -> Option<Box<Node>> {
        // SS: [min, max)
        if min < max {
            let mid = (min + max) / 2;
            let value = input[mid];
            let mut node = Box::new(Node::new(value));
            node.left =
                BinarySearchTree::create_from_sorted_array_internal(&mut node, input, min, mid);
            node.right =
                BinarySearchTree::create_from_sorted_array_internal(&mut node, input, mid + 1, max);
            Some(node)
        } else {
            None
        }
    }

    fn validate(&self) -> bool {
        // SS: using BFS
        if self.root.is_none() {
            true
        } else {
            let mut queue = VecDeque::new();
            queue.push_front(self.root.as_ref().unwrap());
            while queue.is_empty() == false {
                let current_node = queue.pop_back().unwrap();
                let v = current_node.left.as_ref().map_or_else(
                    || true,
                    |left| {
                        queue.push_front(left);
                        left.value <= current_node.value
                    },
                );
                if v == false {
                    return false;
                }
                let v = current_node.right.as_ref().map_or_else(
                    || true,
                    |right| {
                        queue.push_front(right);
                        right.value > current_node.value
                    },
                );
                if v == false {
                    return false;
                }
            }
            true
        }
    }

    fn height(&self) -> u32 {
        if self.root.is_none() {
            0
        } else {
            BinarySearchTree::height_internal(self.root.as_ref().unwrap())
        }
    }

    fn height_internal(node: &Node) -> u32 {
        let left_height = if node.left.is_some() {
            BinarySearchTree::height_internal(node.left.as_ref().unwrap())
        } else {
            0
        };

        let right_height = if node.right.is_some() {
            BinarySearchTree::height_internal(node.right.as_ref().unwrap())
        } else {
            0
        };

        cmp::max(left_height, right_height) + 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bast_validate_1() {
        // Arrange
        let input = [1, 2, 3, 4, 5, 6, 7];

        // Act
        let bst = BinarySearchTree::create_from_sorted_array(&input);

        // Assert
        let is_valid = bst.validate();
        assert_eq!(is_valid, true);
        assert_eq!(bst.height(), 3);
    }
}
