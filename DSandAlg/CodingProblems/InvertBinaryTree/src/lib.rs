use crate::bt::{BinarySearchTree, Node};
use std::collections::VecDeque;

// https://leetcode.com/problems/invert-binary-tree/
mod bt;

fn invert_binary_tree_1(bt: &BinarySearchTree) -> BinarySearchTree {
    // SS: invert tree using BF approach

    // SS: check for empty tree

    let mut inverted_bt = BinarySearchTree::new();
    let root = Box::new(Node::new(bt.root.as_ref().unwrap().value));
    inverted_bt.root = Some(root);

    let mut queue = VecDeque::new();
    queue.push_front((
        bt.root.as_ref().unwrap(),
        inverted_bt.root.as_mut().unwrap(),
    ));
    while queue.is_empty() == false {
        let (primary_node, secondary_node) = queue.pop_back().unwrap();
        let left = &mut secondary_node.left;
        let right = &mut secondary_node.right;

        // SS: duplicate nodes

        {
            // SS: see https://users.rust-lang.org/t/variable-does-not-live-long-enough-with-closure/37935
            let queue = &mut queue;
            primary_node.left.as_ref().map(move |l| {
                let n = Box::new(Node::new(l.value));
                *right = Some(n);
                queue.push_front((l, right.as_mut().unwrap()));
            });
        }

        {
            let queue = &mut queue;
            primary_node.right.as_ref().map(move |r| {
                let n = Box::new(Node::new(r.value));
                *left = Some(n);
                queue.push_front((r, left.as_mut().unwrap()));
            });
        }
    }

    inverted_bt
}

#[cfg(test)]
mod tests {
    use super::*;

    // Solution flattening out, build-up
    // Solution DFS, post-order, bottom-up

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
