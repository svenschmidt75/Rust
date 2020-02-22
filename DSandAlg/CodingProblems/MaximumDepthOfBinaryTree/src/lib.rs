// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use crate::bt::{BinarySearchTree, Node};
use std::cmp;

mod bt;

fn maximum_depth(bt: &BinarySearchTree) -> u8 {
    maximum_depth_internal(&bt.root, 0)
}

fn maximum_depth_internal(bt: &Option<Box<Node>>, depth: u8) -> u8 {
    // SS: using DF approach
    match bt {
        None => depth,
        Some(ref node) => {
            let dl = maximum_depth_internal(&node.left, depth + 1);
            let dr = maximum_depth_internal(&node.right, depth + 1);
            cmp::max(dl, dr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bt::{BinarySearchTree, Node};
    use crate::maximum_depth;

    #[test]
    fn it_works() {
        // Arrange
        let mut bt = BinarySearchTree::new();

        let rl = Some(Box::new(Node::new(15)));
        let rr = Some(Box::new(Node::new(7)));
        let mut r = Box::new(Node::new(20));
        r.left = rl;
        r.right = rr;

        let mut l = Box::new(Node::new(9));

        let mut root = Box::new(Node::new(3));
        root.left = Some(l);
        root.right = Some(r);

        bt.root = Some(root);

        // Act
        let max_depth = maximum_depth(&bt);

        // Assert
        assert_eq!(max_depth, 3);
    }
}
