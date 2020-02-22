use std::collections::VecDeque;

pub struct Node {
    pub value: i64,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i64) {
        if value <= self.value {
            if self.left.is_some() {
                self.left.as_mut().unwrap().insert(value);
            } else {
                let node = Box::new(Node::new(value));
                self.left = Some(node);
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().insert(value);
            } else {
                let node = Box::new(Node::new(value));
                self.right = Some(node);
            }
        }
    }
}

pub struct BinarySearchTree {
    pub root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> BinarySearchTree {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: i64) {
        let node = Box::new(Node::new(value));
        match self.root {
            None => self.root = Some(node),
            Some(ref mut node) => node.insert(value),
        }
    }
}
