#![allow(dead_code)]

use queue::queue::Queue;

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
    T: Ord
{
    /* SS: Here we are tying the lifetime of data which we pass to func to the lifetime
     * of the BinaryTree instance itself.
     */
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

    // SS: take via self -> FnOnce
    //     take via &self -> Fn
    //     take via &mut self -> FnMut
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

    fn visit_levelorder(&'a self, func: &mut FnMut(&'a T)) {
        let mut q = Queue::<&BinaryTree<T>>::new();
        q.enqueue(&self);
        while q.is_empty() == false {
            let tree = q.dequeue();
            match *tree {
                BinaryTree::Empty => {}
                BinaryTree::NonEmpty(ref node) => {
                    func(&node.data);
                    q.enqueue(&node.left);
                    q.enqueue(&node.right);
                }
            }
        }
    }

    fn find_max_element(&self) -> Option<&T> {
        /* Find max. element in a BinaryTree. We could flatten the tree into a list
         * using any of the traversal methods, and then find the max. element in that
         * list, or we traverse in-place.
         */
        match &self {
            &BinaryTree::Empty => None,
            &BinaryTree::NonEmpty(ref node) => {
                let mut elements = vec![&node.data];
                let left_max_element = node.left.find_max_element();
                let right_max_element = node.right.find_max_element();
                if let Some(lval) = left_max_element {
                    elements.push(lval);
                }
                if let Some(rval) = right_max_element {
                    elements.push(rval);
                }
                elements.into_iter().max()
            }
        }
    }

    fn find_element(&self, item: &T) -> bool {
        /* Find element in a BinaryTree. We could flatten the tree into a list
         * using any of the traversal methods, and then find the element in that
         * list, or we traverse in-place.
         */
        match &self {
            &BinaryTree::Empty => false,
            &BinaryTree::NonEmpty(ref node) => {
                if &node.data == item {
                    return true;
                }
                if node.left.find_element(item) {
                    return true;
                }
                node.right.find_element(item)
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

#[test]
fn test_nonempty_visit_levelorder() {
    // Arrange
    let bt = create_tree();
    let mut flattened_list = Vec::new();

    // Act
    bt.visit_levelorder(&mut |value| flattened_list.push(value));

    // Assert
    assert_eq!(vec![&1, &2, &3, &4, &5, &6, &7], flattened_list)
}

#[test]
fn test_nonempty_find_max_element() {
    // Arrange~
    let bt = create_tree();

    // Act
    let max_element = bt.find_max_element().unwrap();

    // Assert
    assert_eq!(7, *max_element)
}

#[test]
fn test_empty_find_max_element() {
    // Arrange~
    let bt: BinaryTree<i32> = BinaryTree::Empty;

    // Act
    let max_element = bt.find_max_element();

    // Assert
    assert_eq!(None, max_element)
}

#[test]
fn test_empty_find_element() {
    // Arrange~
    let bt: BinaryTree<i32> = BinaryTree::Empty;

    // Act
    let max_element = bt.find_element(&5);

    // Assert
    assert_eq!(false, max_element)
}

#[test]
fn test_nonempty_find_element() {
    // Arrange~
    let bt: BinaryTree<i32> = create_tree();

    // Act
    let max_element = bt.find_element(&5);

    // Assert
    assert_eq!(true, max_element)
}
