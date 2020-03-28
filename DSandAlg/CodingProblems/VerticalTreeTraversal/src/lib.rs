// https://practice.geeksforgeeks.org/problems/print-a-binary-tree-in-vertical-order/1

use std::collections::VecDeque;

struct Node {
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: String) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn inorder_traversal(&self, output: &mut Vec<String>) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder_traversal(output);
        }
        output.push(self.value.clone());
        if self.right.is_some() {
            self.right.as_ref().unwrap().inorder_traversal(output);
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

    fn construct_tree_from_inorder(input: &[&str]) -> BinaryTree {
        let mut bt = BinaryTree::new();
        if input.is_empty() {
            bt
        } else {
            let mut value = input[0];
            let mut parent = Box::new(Node::new(value.to_owned()));
            bt.root = Some(parent);

            let mut tape_position = 1;

            let mut queue = VecDeque::new();
            queue.push_front(bt.root.as_mut().unwrap());

            while queue.is_empty() == false && tape_position < input.len() {
                let parent = queue.pop_back().unwrap();

                let value = input[tape_position];
                if value != "N" {
                    let node = Box::new(Node::new(value.to_owned()));
                    parent.left = Some(node);
                    queue.push_front(parent.left.as_mut().unwrap());
                }
                tape_position += 1;

                let value = input[tape_position];
                if value != "N" {
                    let node = Box::new(Node::new(value.to_owned()));
                    parent.right = Some(node);
                    queue.push_front(parent.right.as_mut().unwrap());
                }
                tape_position += 1;
            }
            bt
        }
    }

    fn inorder_traversal(&self) -> Vec<String> {
        let mut result = vec![];
        if self.root.is_some() {
            self.root.as_ref().unwrap().inorder_traversal(&mut result);
        }
        result
    }
}

fn vertical_view(bt: &BinaryTree) -> Vec<Vec<&str>> {
    let mut result = vec![];
    if bt.root.is_some() {
        // SS: traverse in depth-first to find the column for each element
        let mut tmp_result = vec![];
        vertical_view_internal(&mut tmp_result, bt.root.as_ref().unwrap(), 0);

        let &(min, _) = tmp_result.iter().min_by_key(|(column, _)| *column).unwrap();
        let &(max, _) = tmp_result.iter().max_by_key(|(column, _)| *column).unwrap();
        result = vec![vec![]; (max - min + 1) as usize];

        for (column, node_value) in tmp_result {
            let idx = (column - min) as usize;
            result[idx].push(node_value);
        }
    }
    result
}

fn vertical_view_internal<'a>(result: &mut Vec<(i32, &'a str)>, node: &'a Node, column: i32) {
    result.push((column, &node.value));
    if node.left.is_some() {
        vertical_view_internal(result, node.left.as_ref().unwrap(), column - 1);
    }
    if node.right.is_some() {
        vertical_view_internal(result, node.right.as_ref().unwrap(), column + 1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn construct_tree1() {
        // Arrange
        let input = [
            "1", "2", "3", "N", "N", "4", "6", "N", "5", "N", "N", "7", "N",
        ];

        // Act
        let bt = BinaryTree::construct_tree_from_inorder(&input);
        let inorder = bt.inorder_traversal();

        // Assert
        assert_eq!(inorder, vec!["2", "1", "4", "7", "5", "3", "6"]);
    }

    #[test]
    fn vertical_view_tree1() {
        // Arrange
        let input = [
            "1", "2", "3", "N", "N", "4", "6", "N", "5", "N", "N", "7", "N",
        ];
        let bt = BinaryTree::construct_tree_from_inorder(&input);

        // Act
        let vertical_view = vertical_view(&bt);

        // Assert
        assert_eq!(
            vertical_view,
            vec![vec!["2"], vec!["1", "4", "7"], vec!["3", "5"], vec!["6"]]
        );
    }

    #[test]
    fn vertical_view_tree2() {
        // Arrange
        let input = ["1", "2", "3", "4", "5", "N", "6"];
        let bt = BinaryTree::construct_tree_from_inorder(&input);

        // Act
        let vertical_view = vertical_view(&bt);

        // Assert
        assert_eq!(
            vertical_view,
            vec![vec!["4"], vec!["2"], vec!["1", "5"], vec!["3"], vec!["6"]]
        );
    }
}
