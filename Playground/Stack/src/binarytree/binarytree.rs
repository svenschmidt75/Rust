#![allow(dead_code)]

pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    data: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<'a, T> BinaryTree<T>
    where
        T: std::fmt::Display,
{
    fn visit_preorder(&'a self, func: &mut FnMut(&'a T)) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                func(&node.data);
                node.left.visit_preorder(func);
                node.right.visit_preorder(func);
            }
        }
    }

    fn visit_inorder(&'a self, func: &mut FnMut(&'a T)) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                node.left.visit_inorder(func);
                func(&node.data);
                node.right.visit_inorder(func);
            }
        }
    }

    fn visit_postorder(&'a self, func: &mut FnMut(&'a T)) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                node.left.visit_postorder(func);
                node.right.visit_postorder(func);
                func(&node.data);
            }
        }
    }
}

#[test]
fn test_empty_visit_preorder() {
    // Arrange
    let bt = BinaryTree::Empty::<i32>;

    // Act
    // Assert
    bt.visit_preorder(&mut |value| println!("{}", value));
}

fn create_tree() -> BinaryTree<i32> {
    let bt = BinaryTree::NonEmpty(Box::new(TreeNode {
        data: 1,
        left: BinaryTree::NonEmpty(Box::new(TreeNode {
            data: 2,
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                data: 4,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                data: 5,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
        })),
        right: BinaryTree::NonEmpty(Box::new(TreeNode {
            data: 3,
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                data: 6,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                data: 7,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
        })),
    }));
    bt
}

#[test]
fn test_nonempty_visit_preorder() {
    // Arrange
    let bt = create_tree();
    let mut flattened_list = Vec::new();

    // Act
    bt.visit_preorder(&mut |value| flattened_list.push(value));

    // Assert
    assert_eq!(vec![&1, &2, &4, &5, &3, &6, &7], flattened_list)
}

#[test]
fn test_nonempty_visit_inorder() {
    // Arrange
    let bt = create_tree();
    let mut flattened_list = Vec::new();

    // Act
    bt.visit_inorder(&mut |value| flattened_list.push(value));

    // Assert
    assert_eq!(vec![&4, &2, &5, &1, &6, &3, &7], flattened_list)
}

#[test]
fn test_nonempty_visit_postorder() {
    // Arrange
    let bt = create_tree();
    let mut flattened_list = Vec::new();

    // Act
    bt.visit_postorder(&mut |value| flattened_list.push(value));

    // Assert
    assert_eq!(vec![&4, &5, &2, &6, &7, &3, &1], flattened_list)
}
