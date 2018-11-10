#![allow(dead_code)]

use binarytree::binarytree::{BinaryTree
                             , TreeNode};
use std::fmt;

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, " + "),
            Operator::Sub => write!(f, " - "),
            Operator::Mul => write!(f, " * "),
            Operator::Div => write!(f, " / "),
        }
    }
}

enum Expression<'a> {
    Constant(&'a str),
    Operator(Operator)
}

impl<'a> fmt::Display for BinaryTree<Expression<'a>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryTree::NonEmpty(ref node) => {
                match node.as_ref() {
                    TreeNode { data: op, left, right } => {
                        match *op {
                            Expression::Constant(ref c) => write!(f, "{}", c),
                            Expression::Operator(ref op) => {
                                write!(f, "({}{}{})", left, op, right)
                            },
                        }
                    }
                }
            },
            BinaryTree::Empty => write!(f, "empty")
        }
    }
}

trait Expr<T> {
    fn eval(&self) -> T;
}

fn create(postfix: &str) -> BinaryTree<Expression> {
    let items = postfix.split(' ').collect::<Vec<&str>>();
    let mut stack = Vec::new();

    // build expression tree
    for item in items {
        if item.chars().all(char::is_alphanumeric) {
            let node = TreeNode { data: Expression::Constant(item), left: BinaryTree::Empty, right: BinaryTree::Empty };
            stack.push(node);
        } else {
            let d = match item {
                "+" => Expression::Operator(Operator::Add),
                "-" => Expression::Operator(Operator::Sub),
                "*" => Expression::Operator(Operator::Mul),
                "/" => Expression::Operator(Operator::Div),
                _ => panic!("error"),
            };
            // fetch previous 2 nodes
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            let node = TreeNode { data: d, left: BinaryTree::NonEmpty(Box::new(left)), right: BinaryTree::NonEmpty(Box::new(right)) };
            stack.push(node);
        }
    }

    BinaryTree::NonEmpty(Box::new(stack.pop().unwrap()))
}


#[test]
fn test_expressiontree() {
    // Arrange
    let postfix = "A B C * + D /";

    // Act
    let expression_tree = create(postfix);

    // Assert
    println!("{}", expression_tree);
}
