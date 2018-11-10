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

impl Operator {
    fn print(&self) {
        match self {
            Operator::Add => print!(" + "),
            Operator::Sub => print!(" - "),
            Operator::Mul => print!(" * "),
            Operator::Div => print!(" / "),
        }
    }
}

enum Expression<'a> {
    Constant(&'a str),
    Operator(Operator)
}

impl<'a> Expression<'a> {
    fn print(&self) {
        match self {
            Expression::Constant(ref v) => print!("{}", v),
            Expression::Operator(op) => op.print(),
        }
    }
}

impl<'a> fmt::Display for Expression<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
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
    expression_tree.visit_inorder(&mut |v| v.print());
}
