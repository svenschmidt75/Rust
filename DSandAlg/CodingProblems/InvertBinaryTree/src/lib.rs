use crate::bt::{BinarySearchTree, Node};
use std::collections::VecDeque;

// https://leetcode.com/problems/invert-binary-tree/
mod bt;

fn invert_binary_tree_1(bt: &BinarySearchTree) -> BinarySearchTree {
    // SS: invert tree using BF approach

    // SS: check for empty tree
    let mut inverted_bt = BinarySearchTree::new();
    let mut root = Box::new(Node::new(bt.root.as_ref().unwrap().value));
    inverted_bt.root = Some(root);

    let mut queue = VecDeque::new();
    queue.push_front((bt.root.as_ref().unwrap(), inverted_bt.root.as_mut().unwrap()));
    while queue.is_empty() == false {
        let (primary_node, secondary_node) = queue.pop_back().unwrap();

        // SS: duplicate nodes
//        primary_node.left.as_ref().map(|l| {
//            let mut n = Box::new(Node::new(l.value));
//            secondary_node.right = Some(n);
//
//
//            queue.push_front((l, secondary_node.right.as_mut().unwrap()));
//        });

        if primary_node.left.is_some() {
            let l = primary_node.left.as_ref().unwrap();
            let mut n = Box::new(Node::new(l.value));
            secondary_node.right = Some(n);
            queue.push_front((l, secondary_node.right.as_mut().unwrap()));
        }


//        primary_node.right.as_ref().map(|r| {
//            let n = Box::new(Node::new(r.value));
//            secondary_node.right = Some(n);
//        });
        if primary_node.right.is_some() {
            let l = primary_node.right.as_ref().unwrap();
            let mut n = Box::new(Node::new(l.value));
            secondary_node.left = Some(n);
            queue.push_front((l, secondary_node.left.as_mut().unwrap()));
        }
    }

    inverted_bt
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    #[test]
    fn bast_validate_1() {
        // Arrange
        let mut bt = BinarySearchTree::new();

        let rl = Some(Box::new(Node::new(6)));
        let rr = Some(Box::new(Node::new(9)));
        let mut r = Box::new(Node::new(7));
        r.left = rl;
        r.right = rr;

        let ll = Some(Box::new(Node::new(1)));
        let lr = Some(Box::new(Node::new(3)));
        let mut l = Box::new(Node::new(2));
        l.left = ll;
        l.right = lr;

        let mut root = Box::new(Node::new(4));
        root.left = Some(l);
        root.right = Some(r);

        bt.root = Some(root);

        // Act
        let is_valid = invert_binary_tree_1(&bt);

        // Assert
        //assert_eq!(is_valid, false);
    }
}
