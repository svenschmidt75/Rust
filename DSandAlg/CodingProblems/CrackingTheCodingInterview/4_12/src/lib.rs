// Cracking the Coding Interview
// 6th ed, p. 111, ex. 4.12

use std::collections::{HashSet, VecDeque};

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

fn number_of_paths_with_sum(bt: &BinarySearchTree, sum: i64) -> usize {
    // SS: depth-first, post-order
    if bt.root.is_none() {
        0
    } else {
        let mut npaths = 0;

        // SS: find all possible paths
        let root = bt.root.as_ref().unwrap();
        let combined_paths = number_of_paths_with_sum_recursive(root, sum, &mut npaths);

        // SS: check path length together with root
        for path in combined_paths {
            // SS: check path for correct length
            if path_length(&path) == sum {
                npaths += 1;
            }
        }

        npaths
    }
}

fn path_length(path: &Vec<i64>) -> i64 {
    path.iter().sum()
}

fn number_of_paths_with_sum_recursive(node: &Node, sum: i64, npaths: &mut usize) -> Vec<Vec<i64>> {
    // SS: depth-first, post-order
    let mut combined_paths = vec![];

    if node.left.is_some() {
        let left_paths =
            number_of_paths_with_sum_recursive(node.left.as_ref().unwrap(), sum, npaths);
        for mut path in left_paths.into_iter() {
            // SS: check path for correct length
            if path_length(&path) == sum {
                *npaths += 1;
            }
            path.push(node.value);
            combined_paths.push(path);
        }
    }

    if node.right.is_some() {
        let right_paths =
            number_of_paths_with_sum_recursive(node.right.as_ref().unwrap(), sum, npaths);
        for mut path in right_paths.into_iter() {
            // SS: check path for correct length
            if path_length(&path) == sum {
                *npaths += 1;
            }
            path.push(node.value);
            combined_paths.push(path);
        }
    }

    // SS: start a new path with add current node value
    combined_paths.push(vec![node.value]);
    combined_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> BinarySearchTree {
        let mut bt = BinarySearchTree::new();

        let mut r = Box::new(Node::new(3));
        r.left = Some(Box::new(Node::new(6)));
        r.right = Some(Box::new(Node::new(7)));

        let mut l = Box::new(Node::new(2));
        l.left = Some(Box::new(Node::new(4)));
        l.right = Some(Box::new(Node::new(5)));

        let mut root = Box::new(Node::new(1));
        root.left = Some(l);
        root.right = Some(r);

        bt.root = Some(root);

        bt
    }

    #[test]
    fn find_path_6() {
        // Arrange
        let bt = create_tree();

        // Act
        let npaths = number_of_paths_with_sum(&bt, 6);

        // Assert
        assert_eq!(npaths, 2);
    }

    #[test]
    fn find_path_root() {
        // Arrange
        let bt = create_tree();

        // Act
        let npaths = number_of_paths_with_sum(&bt, 1);

        // Assert
        assert_eq!(npaths, 1);
    }
}
