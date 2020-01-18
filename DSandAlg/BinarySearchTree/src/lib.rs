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

    fn validate(&self) -> bool {
        if self.root.is_none() {
            true
        } else {
            let mut queue = VecDeque::new();
            queue.push_front(self.root.as_ref().unwrap());
            while queue.is_empty() == false {
                let current_node = queue.pop_back().unwrap();
                let v = current_node.left.as_ref().map_or_else(
                    || true,
                    |left| {
                        queue.push_front(left);
                        left.value <= current_node.value
                    },
                );
                if v == false {
                    return false;
                }
                let v = current_node.right.as_ref().map_or_else(
                    || true,
                    |right| {
                        queue.push_front(right);
                        right.value > current_node.value
                    },
                );
                if v == false {
                    return false;
                }
            }
            true
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

    fn bfs_flatten_recursive(&self, queue: &mut VecDeque<&Node>, values: &mut Vec<i64>) {
        if queue.is_empty() == false {
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
            self.bfs_flatten_recursive(queue, values);
        }
    }

    fn dfs_inorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_inorder())
    }

    fn dfs_inorder_iteratively(&self) -> Vec<i64> {
        if self.root.is_none() {
            return Vec::<i64>::new();
        }

        let mut values = vec![];
        let mut current_node = self.root.as_ref().unwrap();
        let mut node_stack = vec![];
        node_stack.push(current_node);
        let mut child_pos_stack = vec![];
        child_pos_stack.push(0);
        while node_stack.is_empty() == false {
            current_node = node_stack.pop().unwrap();
            let child_pos = child_pos_stack.pop().unwrap();
            if child_pos == 0 {
                current_node.left.as_ref().map_or_else(
                    || values.push(current_node.value),
                    |left| {
                        node_stack.push(current_node);
                        child_pos_stack.push(1);

                        node_stack.push(left);
                        child_pos_stack.push(0);
                    },
                );
            } else if child_pos == 1 {
                values.push(current_node.value);
                current_node.right.as_ref().map_or_else(
                    || values.push(current_node.value),
                    |right| {
                        node_stack.push(current_node);
                        child_pos_stack.push(2);

                        node_stack.push(right);
                        child_pos_stack.push(0);
                    },
                );
            }
        }
        values
    }

    fn dfs_preorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_preorder())
    }

    fn dfs_preorder_iteratively(&self) -> Vec<i64> {
        if self.root.is_none() {
            return Vec::<i64>::new();
        }

        let mut values = vec![];
        let mut current_node = self.root.as_ref().unwrap();
        let mut node_stack = vec![];
        node_stack.push(current_node);
        let mut child_pos_stack = vec![];
        child_pos_stack.push(0);
        while node_stack.is_empty() == false {
            current_node = node_stack.pop().unwrap();
            let child_pos = child_pos_stack.pop().unwrap();
            if child_pos == 0 {
                values.push(current_node.value);
                current_node.left.as_ref().map(|left| {
                    node_stack.push(current_node);
                    child_pos_stack.push(1);

                    node_stack.push(left);
                    child_pos_stack.push(0);
                });
            } else if child_pos == 1 {
                current_node.right.as_ref().map(|right| {
                    node_stack.push(current_node);
                    child_pos_stack.push(2);

                    node_stack.push(right);
                    child_pos_stack.push(0);
                });
            }
        }
        values
    }

    fn dfs_postorder(&self) -> Vec<i64> {
        self.root
            .as_ref()
            .map_or_else(|| vec![], |r| r.dfs_postorder())
    }

    fn dfs_postorder_iteratively(&self) -> Vec<i64> {
        if self.root.is_none() {
            return Vec::<i64>::new();
        }

        let mut values = vec![];
        let mut current_node = self.root.as_ref().unwrap();
        let mut node_stack = vec![];
        node_stack.push(current_node);
        let mut child_pos_stack = vec![];
        child_pos_stack.push(0);
        while node_stack.is_empty() == false {
            current_node = node_stack.pop().unwrap();
            let child_pos = child_pos_stack.pop().unwrap();
            if child_pos == 0 {
                current_node.left.as_ref().map_or_else(
                    || values.push(current_node.value),
                    |left| {
                        node_stack.push(current_node);
                        child_pos_stack.push(1);

                        node_stack.push(left);
                        child_pos_stack.push(0);
                    },
                );
            } else if child_pos == 1 {
                current_node.right.as_ref().map_or_else(
                    || values.push(current_node.value),
                    |right| {
                        node_stack.push(current_node);
                        child_pos_stack.push(2);

                        node_stack.push(right);
                        child_pos_stack.push(0);
                    },
                );
            } else {
                values.push(current_node.value);
            }
        }
        values
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bfs() {
        // Arrange
        let mut bst = BinarySearchTree::new();
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

        // Act
        let bfs_flattened = bst.bfs_flatten();

        // Assert
        assert_eq!(bfs_flattened, vec![41, 20, 65, 11, 29, 50, 91, 32, 72, 99]);
    }

    #[test]
    fn bfs_recursive() {
        // Arrange
        let mut bst = BinarySearchTree::new();
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

        // Act
        let mut bfs_flattened_recursive = Vec::<i64>::new();
        let mut queue = VecDeque::<&Node>::new();
        queue.push_back(bst.root.as_ref().unwrap());
        bst.bfs_flatten_recursive(&mut queue, &mut bfs_flattened_recursive);

        // Assert
        let bfs_flattened = bst.bfs_flatten();
        assert_eq!(bfs_flattened_recursive, bfs_flattened);
    }

    #[test]
    fn dfs_inorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();
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

        // Act
        let dfs_inorder_flattened = bst.dfs_inorder();

        // Assert
        assert_eq!(
            dfs_inorder_flattened,
            vec![11, 20, 29, 32, 41, 50, 65, 72, 91, 99]
        );
    }

    #[test]
    fn dfs_inorder_iteratively() {
        // Arrange
        let mut bst = BinarySearchTree::new();
        bst.insert(9);
        bst.insert(4);
        bst.insert(20);
        bst.insert(1);
        bst.insert(6);
        bst.insert(15);
        bst.insert(170);

        // Act
        let dfs_inorder_iteratively = bst.dfs_inorder_iteratively();

        // Assert
        let dfs_inorder = bst.dfs_inorder();
        assert_eq!(dfs_inorder_iteratively, dfs_inorder);
    }

    #[test]
    fn dfs_preorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();
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

        // Act
        let dfs_preorder_flattened = bst.dfs_preorder();

        // Assert
        assert_eq!(
            dfs_preorder_flattened,
            vec![41, 20, 11, 29, 32, 65, 50, 91, 72, 99]
        );
    }

    #[test]
    fn dfs_preorder_iteratively() {
        // Arrange
        let mut bst = BinarySearchTree::new();
        bst.insert(9);
        bst.insert(4);
        bst.insert(20);
        bst.insert(1);
        bst.insert(6);
        bst.insert(15);
        bst.insert(170);

        // Act
        let dfs_preorder_iteratively = bst.dfs_preorder_iteratively();

        // Assert
        let dfs_preorder = bst.dfs_preorder();
        assert_eq!(dfs_preorder_iteratively, dfs_preorder);
    }

    #[test]
    fn dfs_postorder() {
        // Arrange
        let mut bst = BinarySearchTree::new();
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

        // Act
        let dfs_postorder_flattened = bst.dfs_postorder();

        // Assert
        assert_eq!(
            dfs_postorder_flattened,
            vec![11, 32, 29, 20, 50, 72, 99, 91, 65, 41]
        );
    }

    #[test]
    fn dfs_postorder_iteratively() {
        // Arrange
        let mut bst = BinarySearchTree::new();
        bst.insert(9);
        bst.insert(4);
        bst.insert(20);
        bst.insert(1);
        bst.insert(6);
        bst.insert(15);
        bst.insert(170);

        // Act
        let dfs_postorder_iteratively = bst.dfs_postorder_iteratively();

        // Assert
        let dfs_postorder = bst.dfs_postorder();
        assert_eq!(dfs_postorder_iteratively, dfs_postorder);
    }

    #[test]
    fn bast_validate_1() {
        // Arrange
        let mut bst = BinarySearchTree::new();
        let rl = Some(Box::new(Node::new(3)));
        let rr = Some(Box::new(Node::new(6)));

        let mut r = Box::new(Node::new(4));
        r.left = rl;
        r.right = rr;

        let l = Some(Box::new(Node::new(1)));

        let mut root = Box::new(Node::new(5));
        root.left = l;
        root.right = Some(r);

        bst.root = Some(root);

        // Act
        let is_valid = bst.validate();

        // Assert
        assert_eq!(is_valid, false);
    }
}
