// Cracking the Coding Interview
// 6th ed, p. 110, ex. 4.4

// https://leetcode.com/problems/balanced-binary-tree/


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

    fn is_balanced(&self) -> bool {
        // SS: Instead of explicitly calculating the heights, we could have a method
        // that terminates early when there is an unbalanced subtree.
        // Runtime is O(v) as we are calculating the height for every node.

        if self.root.is_none() {
            true
        } else {
            let root = self.root.as_ref().unwrap();
            let left_height = if root.left.is_some() {
                BinarySearchTree::height(root.left.as_ref().unwrap())
            } else {
                0
            };
            let right_height = if root.right.is_some() {
                BinarySearchTree::height(root.right.as_ref().unwrap())
            } else {
                0
            };
            (left_height as i32 - right_height as i32).abs() <= 1
        }
    }

    fn height(node: &Node) -> u32 {
        if node.left.is_none() && node.right.is_none() {
            // SS: leaf node has height 1
            1
        } else {
            let left_height = if node.left.is_some() {
                BinarySearchTree::height(node.left.as_ref().unwrap())
            } else {
                0
            };
            let right_height = if node.right.is_some() {
                BinarySearchTree::height(node.right.as_ref().unwrap())
            } else {
                0
            };
            cmp::max(left_height, right_height) + 1
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bt_balanced_1() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        let rl = Some(Box::new(Node::new(15)));
        let rr = Some(Box::new(Node::new(7)));

        let mut r = Box::new(Node::new(20));
        r.left = rl;
        r.right = rr;

        let l = Box::new(Node::new(9));

        let mut root = Box::new(Node::new(3));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        // Act
        let is_balanced = bst.is_balanced();

        // Assert
        assert_eq!(is_balanced, true);
    }

    #[test]
    fn bt_balanced_2() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        let lll = Some(Box::new(Node::new(4)));
        let llr = Some(Box::new(Node::new(4)));

        let mut ll = Box::new(Node::new(3));
        ll.left = lll;
        ll.right = llr;

        let lr = Box::new(Node::new(3));

        let mut l = Box::new(Node::new(2));
        l.left = Some(ll);
        l.right = Some(lr);

        let r = Box::new(Node::new(2));

        let mut root = Box::new(Node::new(3));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        // Act
        let is_balanced = bst.is_balanced();

        // Assert
        assert_eq!(is_balanced, false);
    }
}
