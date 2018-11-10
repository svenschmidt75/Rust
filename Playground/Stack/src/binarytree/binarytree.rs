#![allow(dead_code)]

use queue::queue::Queue;
use stack::stack::Stack;
use std::vec::Vec;
use std::fmt;


// SS: this brings max_by-Key into scope so we can
use std::iter::Iterator;

pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    pub data: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

fn deepest_node_helper<T>(bt: &BinaryTree<T>, level: u32) -> (u32, Option<&TreeNode<T>>) {
    match *bt {
        BinaryTree::NonEmpty(ref node) => {
            let mut v: Vec<(u32, &TreeNode<T>)> = Vec::new();
            let (left_depth, left_node) = deepest_node_helper(&node.left, level + 1);
            left_node.and_then(|val| {
                v.push((left_depth, val));
                Some(true)
            });
            let (right_depth, right_node) = deepest_node_helper(&node.right, level + 1);
            right_node.and_then(|val| {
                v.push((right_depth, val));
                Some(true)
            });
            let a = (level, node.as_ref());
            let (max_level, tree_node) = v.iter().max_by_key(|(v1, _)| *v1).unwrap_or(&a);
            (*max_level, Some(*tree_node))
        },
        BinaryTree::Empty => (level, None),
    }
}

impl<'a, T> BinaryTree<T>
    where
        T: fmt::Display
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

    pub fn visit_inorder(&'a self, func: &mut FnMut(&'a T)) {
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

    fn find_max_element(&self) -> Option<&T> where T: Ord {
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

    fn find_element(&self, item: &T) -> bool where T: Ord {
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

    fn delete_tree(self) {
        // Just by moving, the BinaryTree is destroyed...
        match self {
            BinaryTree::Empty => {},
            BinaryTree::NonEmpty(node) => {
                let tr = *node;
                tr.left.delete_tree();
                tr.right.delete_tree();
            }
        }
    }

    fn size(&self) -> u32 {
        match *self {
            BinaryTree::Empty => 0,
            BinaryTree::NonEmpty(ref node) => {
                let left_size = node.left.size();
                let right_size = node.right.size();
                return 1 + left_size + right_size;
            }
        }
    }

    fn height(&self) -> u32 {
        // height of BT is the length of the path from the root to its
        // deepest node
        match *self {
            BinaryTree::Empty => 0,
            BinaryTree::NonEmpty(ref node) => {
                let left_size = node.left.height();
                let right_size = node.right.height();
                return 1 + std::cmp::max(left_size, right_size);
            }
        }
    }

    fn deepest_node(&self) -> Option<&TreeNode<T>> {
        // Deepest node: length of the path from root to node, where that path length
        // is maximal.
        // could use a max heap...
        let (_, tree_node) = deepest_node_helper(self, 0);
        tree_node
    }

    fn deepest_node2(&self) -> Option<&TreeNode<T>> {
        // Deepest node: length of the path from root to node, where that path length
        // is maximal.
        // Use Breadth-First Search approach
        let mut queue = Queue::new();
        match *self {
            BinaryTree::Empty => return None,
            BinaryTree::NonEmpty(ref node) => {
                queue.enqueue(node.as_ref());
            }
        }
        let mut deepest_node = None;
        while queue.is_empty() == false {
            let n = queue.dequeue();
            deepest_node = Some(n);
            if let BinaryTree::NonEmpty(ref node) = n.left {
                queue.enqueue(node);
            }
            if let BinaryTree::NonEmpty(ref node) = n.right {
                queue.enqueue(node);
            }
        }
        deepest_node
    }

    fn root_to_leaf_paths(&self) {
        let mut stack = Stack::new();
        stack.push(self);
        root_to_leaf_paths_helper(&self, &mut stack);
    }
}

fn root_to_leaf_paths_helper<'a, 'b, T: fmt::Display>(bt: &'a BinaryTree<T>, stack: &'b mut Stack<&'a BinaryTree<T>>) {
    match *bt {
        BinaryTree::Empty => {},
        BinaryTree::NonEmpty(ref node) => {
            if node.is_leaf() {
                let _: Vec<i32> = stack.iter().map(|tree: &&BinaryTree<T>| {
                    match *tree {
                        BinaryTree::Empty => { 1 as i32 },
                        BinaryTree::NonEmpty(ref node) => {
                            print!(" - {}", &node.data);
                            return 1;
                        },
                    }
                }).collect();
                println!();
            } else {
                stack.push(&node.left);
                root_to_leaf_paths_helper(&node.left, stack);
                stack.pop();
                stack.push(&node.right);
                root_to_leaf_paths_helper(&node.right, stack);
                stack.pop();
            }
        },
    }
}


impl<T> TreeNode<T> {
    fn is_leaf(&self) -> bool {
        match self.left {
            BinaryTree::Empty => {},
            BinaryTree::NonEmpty(_) => return false,
        }
        match self.right {
            BinaryTree::Empty => true,
            BinaryTree::NonEmpty(_) => false,
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
    // Arrange
    let bt = create_tree();

    // Act
    let max_element = bt.find_max_element().unwrap();

    // Assert
    assert_eq!(7, *max_element)
}

#[test]
fn test_empty_find_max_element() {
    // Arrange
    let bt: BinaryTree<i32> = BinaryTree::Empty;

    // Act
    let max_element = bt.find_max_element();

    // Assert
    assert_eq!(None, max_element)
}

#[test]
fn test_empty_find_element() {
    // Arrange
    let bt: BinaryTree<i32> = BinaryTree::Empty;

    // Act
    let max_element = bt.find_element(&5);

    // Assert
    assert_eq!(false, max_element)
}

#[test]
fn test_nonempty_find_element() {
    // Arrange
    let bt: BinaryTree<i32> = create_tree();

    // Act
    let max_element = bt.find_element(&5);

    // Assert
    assert_eq!(true, max_element)
}

#[test]
fn test_empty_size() {
    // Arrange
    let bt: BinaryTree<i32> = BinaryTree::Empty;

    // Act
    let size = bt.size();

    // Assert
    assert_eq!(0, size)
}

#[test]
fn test_nonempty_size() {
    // Arrange
    let bt: BinaryTree<i32> = create_tree();

    // Act
    let size = bt.size();

    // Assert
    assert_eq!(7, size)
}

#[test]
fn test_nonempty_height() {
    // Arrange
    let mut bt: BinaryTree<i32> = create_tree();
    match bt {
        BinaryTree::NonEmpty(ref mut n1) => {
            match n1.left {
                BinaryTree::NonEmpty(ref mut n2) => {
                    match n2.left {
                        BinaryTree::NonEmpty(ref mut n3) => {
                            n3.left = BinaryTree::NonEmpty(Box::new(TreeNode {
                                data: 8,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }))
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }

    // Act
    let height = bt.height();

    // Assert
    assert_eq!(4, height)
}

#[test]
fn test_nonempty_deepest_node() {
    // Arrange
    let mut bt: BinaryTree<i32> = create_tree();
    match bt {
        BinaryTree::NonEmpty(ref mut n1) => {
            match n1.right {
                BinaryTree::NonEmpty(ref mut n2) => {
                    match n2.left {
                        BinaryTree::NonEmpty(ref mut n3) => {
                            n3.right = BinaryTree::NonEmpty(Box::new(TreeNode {
                                data: 8,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }))
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }

    // Act
    let node = bt.deepest_node().unwrap();
    // Assert
    assert_eq!(8, node.data);

    // Act
    let node = bt.deepest_node2().unwrap();
    // Assert
    assert_eq!(8, node.data)
}

#[test]
fn test_nonempty_root_to_leaf_paths() {
    // Arrange
    let mut bt: BinaryTree<i32> = create_tree();
    match bt {
        BinaryTree::NonEmpty(ref mut n1) => {
            match n1.right {
                BinaryTree::NonEmpty(ref mut n2) => {
                    match n2.left {
                        BinaryTree::NonEmpty(ref mut n3) => {
                            n3.right = BinaryTree::NonEmpty(Box::new(TreeNode {
                                data: 8,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }))
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }

    // Act
    bt.root_to_leaf_paths();

    // Assert
}
