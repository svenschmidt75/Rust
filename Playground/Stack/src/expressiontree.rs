#![allow(dead_code)]

use binarytree::binarytree::{BinaryTree
                             , TreeNode};
use std::{fmt};
use std::str::{FromStr};
use std::ops::{Add
               , Sub
               , Mul
               , Div};

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

enum Expression<'a, T> {
    Constant(&'a str),
    Operator(Operator),
    Literal(T),
}

impl<'a, T: fmt::Display> fmt::Display for BinaryTree<Expression<'a, T>> {
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
                            Expression::Literal(ref c) => write!(f, "{}", c),
                        }
                    }
                }
            },
            BinaryTree::Empty => write!(f, "empty")
        }
    }
}

trait Eval<T> {
    fn eval(&self) -> T;
}

impl<'a, T> Eval<T> for BinaryTree<Expression<'a, T>>
    where T: Default +
    Copy +
    Add<Output=T> +
    Sub<Output=T> +
    Mul<Output=T> +
    Div<Output=T> {
    fn eval(&self) -> T {
        match self {
            BinaryTree::Empty => Default::default(),
            BinaryTree::NonEmpty(node) => {
                match node.data {
                    Expression::Literal(c) => {
                        c
                    },
                    Expression::Operator(ref op) => {
                        let left = node.left.eval();
                        let right = node.right.eval();
                        match op {
                            Operator::Add => {
                                left + right
                            },
                            Operator::Sub => {
                                left - right
                            },
                            Operator::Mul => {
                                left * right
                            },
                            Operator::Div => {
                                right / left
                            },
                        }
                    },
                    _ => panic!("not defined"),
                }
            },
        }
    }
}

fn create<'a, T>(postfix: &'a str) -> BinaryTree<Expression<T>>
    where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    let items = postfix.split(' ').collect::<Vec<&'a str>>();
    let mut stack = Vec::new();

    // build expression tree
    for item in items {
        if item.chars().all(char::is_alphabetic) {
            let node = TreeNode { data: Expression::Constant(item), left: BinaryTree::Empty, right: BinaryTree::Empty };
            stack.push(node);
        } else if item.chars().all(char::is_numeric) {
            let c = T::from_str(item).unwrap();
            let node = TreeNode { data: Expression::Literal(c), left: BinaryTree::Empty, right: BinaryTree::Empty };
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
    let expression_tree = create::<f32>(postfix);

    // Assert
    println!("{}", expression_tree);
}
#[test]
fn test_eval_expressiontree() {
    // Arrange
    let postfix = "1 2 3 * + 5 /";

    // Act
    let result = create::<f32>(postfix).eval();

    // Assert
    assert_eq!((1_f32 + 2_f32 * 3_f32) / 5_f32, result)
}
