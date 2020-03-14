// https://www.reddit.com/r/CodingProblems/comments/f8b3t1/day_420200223_problem_of_the_day_asked_by_apple/

use crate::Node::Leaf;
use std::collections::VecDeque;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

struct NonLeaf {
    operation: Operation,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl NonLeaf {
    fn new(operation: Operation, left: Option<Box<Node>>, right: Option<Box<Node>>) -> NonLeaf {
        NonLeaf {
            operation,
            left,
            right,
        }
    }
}

enum Node {
    Leaf(f64),
    NonLeaf(NonLeaf),
}

impl Node {
    fn new_leaf(value: f64) -> Node {
        Node::Leaf(value)
    }

    fn new_non_leaf(non_leaf: NonLeaf) -> Node {
        Node::NonLeaf(non_leaf)
    }

    fn evaluate(&self) -> Option<f64> {
        match self {
            Leaf(leaf) => Some(*leaf),
            Node::NonLeaf(nonleaf) => {
                let left = nonleaf
                    .left
                    .as_ref()
                    .map(|l| l.evaluate())
                    .unwrap()
                    .unwrap();
                let right = nonleaf
                    .right
                    .as_ref()
                    .map(|r| r.evaluate())
                    .unwrap()
                    .unwrap();
                let result = match nonleaf.operation {
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                    Operation::Mul => left * right,
                    Operation::Div => left / right,
                };
                Some(result)
            }
        }
    }
}

struct BinaryExpressionTree {
    root: Option<Box<Node>>,
}

impl BinaryExpressionTree {
    fn new() -> BinaryExpressionTree {
        BinaryExpressionTree { root: None }
    }

    fn evaluate(&self) -> Option<f64> {
        if self.root.is_none() {
            None
        } else {
            let value = self.root.as_ref().unwrap().evaluate();
            value
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bast_validate_1() {
        // Arrange
        let ll = Some(Box::new(Node::new_leaf(3.0)));
        let lr = Some(Box::new(Node::new_leaf(2.0)));
        let l = Box::new(Node::new_non_leaf(NonLeaf::new(Operation::Add, ll, lr)));

        let rl = Some(Box::new(Node::new_leaf(4.0)));
        let rr = Some(Box::new(Node::new_leaf(5.0)));
        let r = Box::new(Node::new_non_leaf(NonLeaf::new(Operation::Add, rl, rr)));

        let root = Box::new(Node::new_non_leaf(NonLeaf::new(
            Operation::Mul,
            Some(l),
            Some(r),
        )));

        let mut bst = BinaryExpressionTree::new();
        bst.root = Some(root);

        // Act
        let result = bst.evaluate();

        // Assert
        assert_eq!(result, Some(45.0));
    }
}
