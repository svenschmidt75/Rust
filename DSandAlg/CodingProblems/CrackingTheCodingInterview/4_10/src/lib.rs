// Cracking the Coding Interview
// 6th ed, p. 111, ex. 4.10

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
}

fn find_subtree(main_root: &Node, child_root: &Node) -> bool {
    let mut queue = VecDeque::new();
    queue.push_front(main_root);
    while queue.is_empty() == false {
        let main_node = queue.pop_back().unwrap();
        if trees_equal(main_node, child_root) {
            return true;
        }

        if main_node.left.is_some() {
            queue.push_front(main_node.left.as_ref().unwrap());
        }

        if main_node.right.is_some() {
            queue.push_front(main_node.right.as_ref().unwrap());
        }
    }
    false
}

fn trees_equal(main_node: &Node, child_node: &Node) -> bool {
    if main_node.value == child_node.value {
        let left_subtree_equal = if main_node.left.is_none() && child_node.left.is_some() {
            false
        } else if main_node.left.is_some() && child_node.left.is_none() {
            false
        } else if main_node.left.is_none() && child_node.left.is_none() {
            true
        } else {
            trees_equal(
                main_node.left.as_ref().unwrap(),
                child_node.left.as_ref().unwrap(),
            )
        };

        let right_subtree_equal = if main_node.right.is_none() && child_node.right.is_some() {
            false
        } else if main_node.right.is_some() && child_node.right.is_none() {
            false
        } else if main_node.right.is_none() && child_node.right.is_none() {
            true
        } else {
            trees_equal(
                main_node.right.as_ref().unwrap(),
                child_node.right.as_ref().unwrap(),
            )
        };

        left_subtree_equal && right_subtree_equal
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_main_tree() -> BinarySearchTree {
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
    fn create_child_tree() -> BinarySearchTree {
        let mut bst = BinarySearchTree::new();

        let r = Box::new(Node::new(16));
        let l = Box::new(Node::new(15));

        let mut root = Box::new(Node::new(12));
        root.left = Some(l);
        root.right = Some(r);

        bst.root = Some(root);

        bst
    }

    #[test]
    fn test_subtree_true() {
        // Arrange
        let t1 = create_main_tree();
        let t2 = create_child_tree();

        // Act
        let is_subtree = find_subtree(t1.root.as_ref().unwrap(), t2.root.as_ref().unwrap());

        // Assert
        assert_eq!(is_subtree, true);
    }

    #[test]
    fn test_subtree_false() {
        // Arrange
        let t1 = create_main_tree();
        let t2 = create_child_tree();
        let t1_node = t1.root.as_ref().unwrap().left.as_ref().unwrap();

        // Act
        let is_subtree = find_subtree(t1_node, t2.root.as_ref().unwrap());

        // Assert
        assert_eq!(is_subtree, false);
    }
}
