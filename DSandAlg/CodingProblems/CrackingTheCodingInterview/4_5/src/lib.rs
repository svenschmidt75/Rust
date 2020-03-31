// Cracking the Coding Interview
// 6th ed, p. 110, ex. 4.5

// https://leetcode.com/problems/validate-binary-search-tree/

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

    fn validate(&self) -> bool {
        // SS: Check each node for the BST property...
        if self.root.is_none() {
            true
        } else {
            BinarySearchTree::validate_internal(self.root.as_ref().unwrap())
        }
    }

    fn validate_internal(node: &Node) -> bool {
        // SS: check BST property of children first
        let left_child = node.left.as_ref();
        if left_child.is_some() && left_child.unwrap().value > node.value {
            return false;
        }

        let right_child = node.right.as_ref();
        if right_child.is_some() && right_child.unwrap().value < node.value {
            return false;
        }

        // SS: check BST property of left subtree
        let left_valid = if left_child.is_some() {
            BinarySearchTree::validate_internal(left_child.unwrap())
        } else {
            true
        };

        // SS: check BST property of right subtree
        let right_valid = if right_child.is_some() {
            BinarySearchTree::validate_internal(right_child.unwrap())
        } else {
            true
        };

        left_valid && right_valid
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bst_validate_1() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        let l = Box::new(Node::new(1));
        let r = Box::new(Node::new(3));

        let mut root = Box::new(Node::new(2));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        // Act
        let is_bst = bst.validate();

        // Assert
        assert_eq!(is_bst, true);
    }

    #[test]
    fn bt_balanced_2() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        let rr = Box::new(Node::new(6));
        let rl = Box::new(Node::new(3));

        let mut r = Box::new(Node::new(4));
        r.left = Some(rl);
        r.right = Some(rr);

        let l = Box::new(Node::new(1));

        let mut root = Box::new(Node::new(5));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        // Act
        let is_bst = bst.validate();

        // Assert
        assert_eq!(is_bst, false);
    }
}
