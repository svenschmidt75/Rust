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

    fn dfs_inorder(&self) -> Vec<i64> {
        let mut values = vec![];
        self.dfs_inorder_internal(&mut values);
        values
    }

    fn dfs_inorder_internal(&self, values: &mut Vec<i64>) {
        self.left
            .as_ref()
            .map(|left| left.dfs_inorder_internal(values));
        values.push(self.value);
        self.right
            .as_ref()
            .map(|right| right.dfs_inorder_internal(values));
    }

    fn dfs_preorder(&self) -> Vec<i64> {
        let mut values = vec![];
        self.dfs_preorder_internal(&mut values);
        values
    }

    fn dfs_preorder_internal(&self, values: &mut Vec<i64>) {
        values.push(self.value);
        self.left
            .as_ref()
            .map(|left| left.dfs_preorder_internal(values));
        self.right
            .as_ref()
            .map(|right| right.dfs_preorder_internal(values));
    }

    fn dfs_postorder(&self) -> Vec<i64> {
        let mut values = vec![];
        self.dfs_postorder_internal(&mut values);
        values
    }

    fn dfs_postorder_internal(&self, values: &mut Vec<i64>) {
        self.left
            .as_ref()
            .map(|left| left.dfs_postorder_internal(values));
        self.right
            .as_ref()
            .map(|right| right.dfs_postorder_internal(values));
        values.push(self.value);
    }
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> BinarySearchTree {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: i64) {
        let node = Box::new(Node::new(value));
        match self.root {
            None => self.root = Some(node),
            Some(ref mut node) => node.insert(value),
        }
    }

    fn bfs_flatten(&self) -> Vec<i64> {
        if self.root.is_none() {
            vec![]
        } else {
            let mut values = vec![];
            let mut queue = VecDeque::new();
            queue.push_front(self.root.as_ref().unwrap());
            while queue.is_empty() == false {
                let current_node = queue.pop_back().unwrap();
                values.push(current_node.value);
                current_node
                    .left
                    .as_ref()
                    .map(|left_child| queue.push_front(left_child));
                current_node
                    .right
                    .as_ref()
                    .map(|right_child| queue.push_front(right_child));
            }
            values
        }
    }

    fn dfs_inorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_inorder())
    }

    fn dfs_inorder_iteratively(&self) -> Vec<i64> {
        let mut current_node = self.root.as_ref();
        let mut stack = vec![];
        stack.push(current_node);
        while stack.is_empty() == false {
            current_node = stack.pop().unwrap();
        }
        vec![]
    }

    fn dfs_preorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_preorder())
    }

    fn dfs_postorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_postorder())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bfs() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        bst.insert(41);
        bst.insert(20);
        bst.insert(11);
        bst.insert(29);
        bst.insert(32);
        bst.insert(65);
        bst.insert(50);
        bst.insert(91);
        bst.insert(72);
        bst.insert(99);

        // Assert
        let bfs_flattened = bst.bfs_flatten();
        assert_eq!(bfs_flattened, vec![41, 20, 65, 11, 29, 50, 91, 32, 72, 99]);
    }

    #[test]
    fn dfs_inorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        bst.insert(41);
        bst.insert(20);
        bst.insert(11);
        bst.insert(29);
        bst.insert(32);
        bst.insert(65);
        bst.insert(50);
        bst.insert(91);
        bst.insert(72);
        bst.insert(99);

        // Assert
        let dfs_inorder_flattened = bst.dfs_inorder();
        assert_eq!(
            dfs_inorder_flattened,
            vec![11, 20, 29, 32, 41, 50, 65, 72, 91, 99]
        );
    }

    #[test]
    fn dfs_preorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        bst.insert(41);
        bst.insert(20);
        bst.insert(11);
        bst.insert(29);
        bst.insert(32);
        bst.insert(65);
        bst.insert(50);
        bst.insert(91);
        bst.insert(72);
        bst.insert(99);

        // Assert
        let dfs_preorder_flattened = bst.dfs_preorder();
        assert_eq!(
            dfs_preorder_flattened,
            vec![41, 20, 11, 29, 32, 65, 50, 91, 72, 99]
        );
    }

    #[test]
    fn dfs_postorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        bst.insert(41);
        bst.insert(20);
        bst.insert(11);
        bst.insert(29);
        bst.insert(32);
        bst.insert(65);
        bst.insert(50);
        bst.insert(91);
        bst.insert(72);
        bst.insert(99);

        // Assert
        let dfs_postorder_flattened = bst.dfs_postorder();
        assert_eq!(
            dfs_postorder_flattened,
            vec![11, 32, 29, 20, 50, 72, 99, 91, 65, 41]
        );
    }

}
