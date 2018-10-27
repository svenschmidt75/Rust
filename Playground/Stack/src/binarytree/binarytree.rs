pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    data: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTree<T>
where
    T: std::fmt::Display,
{
    fn visit_preorder<'a>(&'a self, func: &mut FnMut(&'a T)) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                func(&node.data);
                node.left.visit_preorder(func);
                node.right.visit_preorder(func);
            }
        }
    }

    //    fn visit_inorder(&self) {
    //
    //    }
    //
    //    fn visit_postorder(&self) {
    //
    //    }
}

#[test]
fn test_empty_visit_preorder() {
    // Arrange
    let bt = BinaryTree::Empty::<i32>;

    // Act
    // Assert
    bt.visit_preorder(&mut |value| println!("{}", value));
}

#[test]
fn test_nonempty_visit_preorder() {
    // Arrange
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
    let mut flattened_list = Vec::new();

    // Act
    bt.visit_preorder(&mut |value| flattened_list.push(value));

    // Assert
    assert_eq!(vec![&1, &2, &4, &5, &3, &6, &7], flattened_list)
}
