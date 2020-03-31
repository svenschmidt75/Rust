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

    fn find_path(&self, from: i64, to: i64) -> Vec<i64> {
        if self.root.is_none() {
            vec![]
        } else {
            let mut path = vec![];
            BinarySearchTree::find_path_recursive(self.root.as_ref().unwrap(), from, to, &mut path);
            path
        }
    }

    fn find_path_recursive(parent: &Node, from: i64, to: i64, path: &mut Vec<i64>) -> bool {
        // SS: base case for recursion
        if parent.value == from || parent.value == to {
            path.push(parent.value);
            true
        } else {
            let mut left_path = vec![];
            let mut left_result = false;
            if parent.left.is_some() {
                left_result = BinarySearchTree::find_path_recursive(
                    parent.left.as_ref().unwrap(),
                    from,
                    to,
                    &mut left_path,
                );
            }

            let mut right_path = vec![];
            let mut right_result = false;
            if parent.right.is_some() {
                right_result = BinarySearchTree::find_path_recursive(
                    parent.right.as_ref().unwrap(),
                    from,
                    to,
                    &mut right_path,
                );
            }

            // SS: if both left_path and right_path are non-empty, this is the 1st common ancestor
            if left_path.is_empty() == false && right_path.is_empty() == false {
                path.append(&mut left_path);
                path.push(parent.value);

                // SS: add right_path in reversed order
                right_path.reverse();
                path.append(&mut right_path);
                false
            } else {
                path.append(&mut left_path);
                path.append(&mut right_path);
                if left_result || right_result {
                    path.push(parent.value);
                }
                left_result || right_result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bt_find_path() {
        // Arrange
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

        // Act
        let path = bst.find_path(14, 12);

        // // Assert
        assert_eq!(path, vec![14, 10, 6, 3, 7, 12]);
    }
}
