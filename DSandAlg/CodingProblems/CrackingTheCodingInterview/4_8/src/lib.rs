// Cracking the Coding Interview
// 6th ed, p. 110, ex. 4.8

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

    fn find_first_common_ancestor(&self, node1: i64, node2: i64) -> Option<i64> {
        if self.root.is_none() {
            None
        } else {
            let mut ancestor = 0i64;
            BinarySearchTree::find_first_common_ancestor_recursive(
                self.root.as_ref().unwrap(),
                node1,
                node2,
                &mut ancestor,
            );
            Some(ancestor)
        }
    }

    fn find_first_common_ancestor_recursive(
        parent: &Node,
        node1: i64,
        node2: i64,
        ancestor: &mut i64,
    ) -> bool {
        let mut left_subtree_contains_node = false;
        if parent.left.is_some() {
            left_subtree_contains_node = BinarySearchTree::find_first_common_ancestor_recursive(
                parent.left.as_ref().unwrap(),
                node1,
                node2,
                ancestor,
            );
        }

        let mut right_subtree_contains_node = false;
        if parent.right.is_some() {
            right_subtree_contains_node = BinarySearchTree::find_first_common_ancestor_recursive(
                parent.right.as_ref().unwrap(),
                node1,
                node2,
                ancestor,
            );
        }

        let mut add = left_subtree_contains_node || right_subtree_contains_node;

        if parent.value == node1 || parent.value == node2 {
            if left_subtree_contains_node || right_subtree_contains_node || node1 == node2 {
                *ancestor = parent.value;
            }
            add = true;
        } else if left_subtree_contains_node && right_subtree_contains_node {
            // SS: this is the common ancestor
            *ancestor = parent.value;
            add = false;
        }

        add
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> BinarySearchTree {
        let mut bst = BinarySearchTree::new();

        let lrl = Box::new(Node::new(8));

        let mut lr = Box::new(Node::new(5));
        lr.left = Some(lrl);

        let ll = Box::new(Node::new(4));

        let mut l = Box::new(Node::new(2));
        l.left = Some(ll);
        l.right = Some(lr);

        let rlrrl = Box::new(Node::new(17));
        let rlrrr = Box::new(Node::new(18));

        let mut rlrr = Box::new(Node::new(14));
        rlrr.left = Some(rlrrl);
        rlrr.right = Some(rlrrr);

        let rlrl = Box::new(Node::new(13));

        let mut rlr = Box::new(Node::new(10));
        rlr.left = Some(rlrl);
        rlr.right = Some(rlrr);

        let rll = Box::new(Node::new(9));

        let rrrr = Box::new(Node::new(16));
        let rrrl = Box::new(Node::new(15));

        let mut rrr = Box::new(Node::new(12));
        rrr.left = Some(rrrl);
        rrr.right = Some(rrrr);

        let rrl = Box::new(Node::new(11));

        let mut rr = Box::new(Node::new(7));
        rr.left = Some(rrl);
        rr.right = Some(rrr);

        let mut rl = Box::new(Node::new(6));
        rl.left = Some(rll);
        rl.right = Some(rlr);

        let mut r = Box::new(Node::new(3));
        r.left = Some(rl);
        r.right = Some(rr);

        let mut root = Box::new(Node::new(1));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        bst
    }

    #[test]
    fn bt_find_path_different_subtrees() {
        // Arrange
        let bst = create_tree();

        // Act
        let result = bst.find_first_common_ancestor(14, 12).unwrap();

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn bt_find_path_different_subtrees_2() {
        // Arrange
        let bst = create_tree();

        // Act
        let result = bst.find_first_common_ancestor(8, 17).unwrap();

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn bt_find_path_left_right_subtree() {
        // Arrange
        let bst = create_tree();

        // Act
        let result = bst.find_first_common_ancestor(17, 3).unwrap();

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn bt_find_path_right_subtree() {
        // Arrange
        let bst = create_tree();

        // Act
        let result = bst.find_first_common_ancestor(14, 6).unwrap();

        // Assert
        assert_eq!(result, 6);
    }

    #[test]
    fn bt_find_path_single_vertex() {
        // Arrange
        let bst = create_tree();

        // Act
        let result = bst.find_first_common_ancestor(14, 14).unwrap();

        // Assert
        assert_eq!(result, 14);
    }
}
