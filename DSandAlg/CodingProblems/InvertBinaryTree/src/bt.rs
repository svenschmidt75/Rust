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

#[cfg(test)]
mod tests {

    use super::*;

    //    #[test]
    //    fn bfs() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(41);
    //        bst.insert(20);
    //        bst.insert(11);
    //        bst.insert(29);
    //        bst.insert(32);
    //        bst.insert(65);
    //        bst.insert(50);
    //        bst.insert(91);
    //        bst.insert(72);
    //        bst.insert(99);
    //
    //        // Act
    //        let bfs_flattened = bst.bfs_flatten();
    //
    //        // Assert
    //        assert_eq!(bfs_flattened, vec![41, 20, 65, 11, 29, 50, 91, 32, 72, 99]);
    //    }
    //
    //    #[test]
    //    fn bfs_recursive() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(41);
    //        bst.insert(20);
    //        bst.insert(11);
    //        bst.insert(29);
    //        bst.insert(32);
    //        bst.insert(65);
    //        bst.insert(50);
    //        bst.insert(91);
    //        bst.insert(72);
    //        bst.insert(99);
    //
    //        // Act
    //        let mut bfs_flattened_recursive = Vec::<i64>::new();
    //        let mut queue = VecDeque::<&Node>::new();
    //        queue.push_back(bst.root.as_ref().unwrap());
    //        bst.bfs_flatten_recursive(&mut queue, &mut bfs_flattened_recursive);
    //
    //        // Assert
    //        let bfs_flattened = bst.bfs_flatten();
    //        assert_eq!(bfs_flattened_recursive, bfs_flattened);
    //    }
    //
    //    #[test]
    //    fn dfs_inorder() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(41);
    //        bst.insert(20);
    //        bst.insert(11);
    //        bst.insert(29);
    //        bst.insert(32);
    //        bst.insert(65);
    //        bst.insert(50);
    //        bst.insert(91);
    //        bst.insert(72);
    //        bst.insert(99);
    //
    //        // Act
    //        let dfs_inorder_flattened = bst.dfs_inorder();
    //
    //        // Assert
    //        assert_eq!(
    //            dfs_inorder_flattened,
    //            vec![11, 20, 29, 32, 41, 50, 65, 72, 91, 99]
    //        );
    //    }
    //
    //    #[test]
    //    fn dfs_inorder_iteratively() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(9);
    //        bst.insert(4);
    //        bst.insert(20);
    //        bst.insert(1);
    //        bst.insert(6);
    //        bst.insert(15);
    //        bst.insert(170);
    //
    //        // Act
    //        let dfs_inorder_iteratively = bst.dfs_inorder_iteratively();
    //
    //        // Assert
    //        let dfs_inorder = bst.dfs_inorder();
    //        assert_eq!(dfs_inorder_iteratively, dfs_inorder);
    //    }
    //
    //    #[test]
    //    fn dfs_preorder() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(41);
    //        bst.insert(20);
    //        bst.insert(11);
    //        bst.insert(29);
    //        bst.insert(32);
    //        bst.insert(65);
    //        bst.insert(50);
    //        bst.insert(91);
    //        bst.insert(72);
    //        bst.insert(99);
    //
    //        // Act
    //        let dfs_preorder_flattened = bst.dfs_preorder();
    //
    //        // Assert
    //        assert_eq!(
    //            dfs_preorder_flattened,
    //            vec![41, 20, 11, 29, 32, 65, 50, 91, 72, 99]
    //        );
    //    }
    //
    //    #[test]
    //    fn dfs_preorder_iteratively() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(9);
    //        bst.insert(4);
    //        bst.insert(20);
    //        bst.insert(1);
    //        bst.insert(6);
    //        bst.insert(15);
    //        bst.insert(170);
    //
    //        // Act
    //        let dfs_preorder_iteratively = bst.dfs_preorder_iteratively();
    //
    //        // Assert
    //        let dfs_preorder = bst.dfs_preorder();
    //        assert_eq!(dfs_preorder_iteratively, dfs_preorder);
    //    }
    //
    //    #[test]
    //    fn dfs_preorder_iteratively2() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(9);
    //        bst.insert(4);
    //        bst.insert(20);
    //        bst.insert(1);
    //        bst.insert(6);
    //        bst.insert(15);
    //        bst.insert(170);
    //
    //        // Act
    //        let dfs_preorder_iteratively = bst.dfs_preorder_iteratively2();
    //
    //        // Assert
    //        let dfs_preorder = bst.dfs_preorder();
    //        assert_eq!(dfs_preorder_iteratively, dfs_preorder);
    //    }
    //
    //    #[test]
    //    fn dfs_postorder() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(41);
    //        bst.insert(20);
    //        bst.insert(11);
    //        bst.insert(29);
    //        bst.insert(32);
    //        bst.insert(65);
    //        bst.insert(50);
    //        bst.insert(91);
    //        bst.insert(72);
    //        bst.insert(99);
    //
    //        // Act
    //        let dfs_postorder_flattened = bst.dfs_postorder();
    //
    //        // Assert
    //        assert_eq!(
    //            dfs_postorder_flattened,
    //            vec![11, 32, 29, 20, 50, 72, 99, 91, 65, 41]
    //        );
    //    }
    //
    //    #[test]
    //    fn dfs_postorder_iteratively() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        bst.insert(9);
    //        bst.insert(4);
    //        bst.insert(20);
    //        bst.insert(1);
    //        bst.insert(6);
    //        bst.insert(15);
    //        bst.insert(170);
    //
    //        // Act
    //        let dfs_postorder_iteratively = bst.dfs_postorder_iteratively();
    //
    //        // Assert
    //        let dfs_postorder = bst.dfs_postorder();
    //        assert_eq!(dfs_postorder_iteratively, dfs_postorder);
    //    }
    //
    //    #[test]
    //    fn bast_validate_1() {
    //        // Arrange
    //        let mut bst = BinarySearchTree::new();
    //        let rl = Some(Box::new(Node::new(3)));
    //        let rr = Some(Box::new(Node::new(6)));
    //
    //        let mut r = Box::new(Node::new(4));
    //        r.left = rl;
    //        r.right = rr;
    //
    //        let l = Some(Box::new(Node::new(1)));
    //
    //        let mut root = Box::new(Node::new(5));
    //        root.left = l;
    //        root.right = Some(r);
    //
    //        bst.root = Some(root);
    //
    //        // Act
    //        let is_valid = bst.validate();
    //
    //        // Assert
    //        assert_eq!(is_valid, false);
    //    }
}
