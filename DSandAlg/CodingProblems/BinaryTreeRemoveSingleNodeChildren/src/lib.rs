// https://www.reddit.com/r/CodingProblems/comments/fbw3b2/day_62020031_problem_of_the_day_asked_by_google/

use std::collections::VecDeque;
use std::ptr;

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

    fn remove_single_node_children(&mut self) {
        let mut queue = VecDeque::new();

        if let Some(root) = self.root.as_mut() {
            let parent = root.as_mut() as *mut Node;

            if let Some(child) = root.left.as_mut() {
                let c = child.as_mut() as *mut Node;
                queue.push_front((parent, c));
            }

            if let Some(child) = root.right.as_mut() {
                let c = child.as_mut() as *mut Node;
                queue.push_front((parent, c));
            }
        }

        while queue.is_empty() == false {
            let (parent, child) = queue.pop_back().unwrap();

            let mut single_child = None;

            unsafe {
                if (*child).left.is_some() && (*child).right.is_none() {
                    single_child = (*child).left.take();
                }
            }

            unsafe {
                if (*child).left.is_none() && (*child).right.is_some() {
                    single_child = (*child).right.take();
                }
            }

            if let None = single_child {
                // SS: either no children or two

                unsafe {
                    if let Some(l) = (*child).left.as_mut() {
                        queue.push_front((child, l.as_mut() as *mut Node));
                    }
                }

                unsafe {
                    if let Some(r) = (*child).right.as_mut() {
                        queue.push_front((child, r.as_mut() as *mut Node));
                    }
                }
            } else {
                // SS: child has one child
                unsafe {
                    if let Some(l) = (*parent).left.as_mut() {
                        if ptr::eq(l.as_ref(), child) {
                            // SS: replace parent's left child with single child
                            (*parent).left = single_child;
                            queue.push_front((parent, l.as_mut()));
                        }
                    } else {
                        if let Some(r) = (*parent).right.as_mut() {
                            if ptr::eq(r.as_ref(), child) {
                                // SS: replace parent's right child with single child
                                (*parent).right = single_child;
                                queue.push_front((parent, r.as_mut()));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bast_validate_1() {
        // Arrange
        let mut bt = BinarySearchTree::new();

        let mut l = Box::new(Node::new(2));
        l.left = Some(Box::new(Node::new(0)));

        let mut r = Box::new(Node::new(3));
        r.left = Some(Box::new(Node::new(9)));
        r.right = Some(Box::new(Node::new(4)));

        let mut root = Box::new(Node::new(1));
        root.left = Some(l);
        root.right = Some(r);

        bt.root = Some(root);

        // Act
        bt.remove_single_node_children();

        // Assert
        //        assert_eq!(is_valid, false);
    }
}
