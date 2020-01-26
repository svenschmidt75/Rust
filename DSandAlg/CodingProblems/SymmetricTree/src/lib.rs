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

struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    fn new() -> BinaryTree {
        BinaryTree { root: None }
    }

    fn is_symmetric(&self) -> bool {
        // SS: use BFS to traverse the binary tree

        self.root.as_ref().map_or(true, |r| {
            let mut queue = VecDeque::new();
            queue.push_back((r.left.as_ref(), r.right.as_ref()));
            while queue.is_empty() == false {
                let (left, right) = queue.pop_back().unwrap();
                if left.is_some() && right.is_some() {
                    let result = left.map_or(false, |l| {
                        right.map_or(false, |r| {
                            if l.value != r.value {
                                return false;
                            }
                            queue.push_back((l.left.as_ref(), r.right.as_ref()));
                            queue.push_back((l.right.as_ref(), r.left.as_ref()));
                            true
                        })
                    });

                    if result == false {
                        return false;
                    }
                } else if left.is_none() && right.is_none() {
                    // SS: this case is ok
                } else {
                    // SS: uneven nodes, cannot be symmetric
                    return false;
                }
            }

            true
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let lll = Some(Box::new(Node::new(7)));
        let llr = Some(Box::new(Node::new(9)));
        let mut ll = Box::new(Node::new(3));
        ll.left = lll;
        ll.right = llr;

        let lrl = Some(Box::new(Node::new(3)));
        let lrr = Some(Box::new(Node::new(2)));
        let mut lr = Box::new(Node::new(4));
        lr.left = lrl;
        lr.right = lrr;

        let rll = Some(Box::new(Node::new(2)));
        let rlr = Some(Box::new(Node::new(3)));
        let mut rl = Box::new(Node::new(4));
        rl.left = rll;
        rl.right = rlr;

        let rrl = Some(Box::new(Node::new(9)));
        let rrr = Some(Box::new(Node::new(7)));
        let mut rr = Box::new(Node::new(3));
        rr.left = rrl;
        rr.right = rrr;

        let mut l = Box::new(Node::new(2));
        l.left = Some(ll);
        l.right = Some(lr);

        let mut r = Box::new(Node::new(2));
        r.left = Some(rl);
        r.right = Some(rr);

        let mut root = Box::new(Node::new(1));
        root.left = Some(l);
        root.right = Some(r);

        let mut bst = BinaryTree::new();
        bst.root = Some(root);

        // Act
        let is_symmetric = bst.is_symmetric();

        // Assert
        assert_eq!(is_symmetric, true);
    }

    #[test]
    fn test2() {
        // Arrange
        let rr = Box::new(Node::new(3));
        let mut r = Box::new(Node::new(2));
        r.right = Some(rr);

        let lr = Box::new(Node::new(3));
        let mut l = Box::new(Node::new(2));
        l.right = Some(lr);

        let mut root = Box::new(Node::new(1));
        root.left = Some(l);
        root.right = Some(r);

        let mut bst = BinaryTree::new();
        bst.root = Some(root);

        // Act
        let is_symmetric = bst.is_symmetric();

        // Assert
        assert_eq!(is_symmetric, false);
    }
}
