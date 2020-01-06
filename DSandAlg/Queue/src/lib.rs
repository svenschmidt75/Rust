use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{BorrowMut, Borrow};

struct Node {
    value: u64,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: u64) -> Node {
        Node { value, next: None }
    }

    fn find_leaf_mut(&mut self) -> &mut Node {
        let mut current_node = self;
        loop {
            if current_node.next.is_none() {
                return current_node;
            } else {
                let mut a = current_node.next.as_ref().unwrap().clone();
                let mut b = a.borrow_mut().get_mut();


                current_node = b;



            }
        }
    }

    fn find(&self, value: u64) -> Option<&Node> {
        if self.value == value {
            return Some(self);
        }
        match self.next {
            None => None,
            Some(ref node) => node.deref().borrow().find(value),
        }
    }

    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
        if self.value == value {
            return Some(self);
        }
        match self.next {
            None => None,
            Some(ref node) => {
                node.deref().borrow().find_mut(value)
            },
        }
    }

    fn remove(&mut self, value: u64) {
        // SS: find the parent node of the node to delete
        let mut parent = self;
        while parent.next.is_some() && parent.next.unwrap().deref().borrow().value != value {
            parent = parent.next.unwrap().deref().borrow_mut().deref_mut();
        }
        if let Some(ref child) = parent.next {
//            assert_eq!(child.deref().value, value);
            let child_next = child.deref().borrow_mut().next.take();
            parent.next = child_next;
        }
    }

    fn append(&mut self, value: u64) -> Rc<RefCell<Node>> {
        let mut leaf = self.find_leaf_mut();
        let node = Rc::new(RefCell::new(Node::new(value)));
        leaf.next = Some(node.clone());
        node
    }

    fn print(&self) {
        print!("{} ", self.value);
        match self.next {
            None => {}
            Some(ref next) => {
                next.deref().borrow().print();
            }
        }
    }
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: u64) {
        match self.head {
            None => {
                let node = Rc::new(RefCell::new(Node::new(value)));
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
            Some(ref node) => {
                let new_tail = node.deref().borrow_mut().append(value);
                self.tail = Some(new_tail);
            }
        }
    }

    fn prepend(&mut self, value: u64) {
        let mut node = RefCell::new(Node::new(value));
        let head = self.head.take();
        node.borrow_mut().next = match head {
            None => None,
            Some(ref next) => Some(next.clone()),
        };
        let rc = Rc::new(node);
        if head.is_none() {
            // SS: set tail as well
            self.tail = Some(rc.clone());
        }
        self.head = Some(rc.clone());
    }

    fn remove(&mut self, value: u64) {
        if let Some(ref head) = self.head {
            if head.deref().borrow().value == value {
                // SS: remove head

                // SS: next node after head
                let next = self.head.unwrap().deref().borrow().next.take();
                if let Some(n) = next {
                    self.head = Some(n);
                }
            } else {
                head.deref().borrow().remove(value);
            }
        }
    }

    fn find(&self, value: u64) -> Option<&Node> {
        match self.head {
            None => None,
            Some(ref node) => node.deref().borrow().find(value),
        }
    }

    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
        match self.head {
            None => None,
            Some(ref node) => {
                node.deref().borrow_mut().find_mut(value)
            },
        }
    }

    fn length(&self) -> usize {
        if let None = self.head {
            0
        } else {
            let mut count = 1;
            let mut node = self.head.unwrap().deref().borrow().deref();
            while node.next.is_some() {
                count = count + 1;
                node = node.next.unwrap().deref().borrow().deref();
            }
            count
        }
    }

    fn print(&self) {
        match self.head {
            None => println!("empty"),
            Some(ref node) => {
                node.deref().borrow().print();
                println!()
            }
        }
    }
}

struct Queue {
    linked_list: LinkedList,
}

impl Queue {
    fn new() -> Queue {
        Queue {
            linked_list: LinkedList::new(),
        }
    }

    fn enqueue(&mut self, value: u64) {
        // SS: prepend an element
        self.linked_list.append(value);
    }

    fn dequeue(&mut self) -> Option<u64> {
        let mut head = self.linked_list.head.take();
        match head {
            None => None,
            Some(ref rc) => {
                // SS: set new head to next node
                self.linked_list.head = rc.deref().borrow().next;
                Some(rc.deref().borrow().value)
            }
        }
    }

    fn peek(&self) -> Option<u64> {
        match self.linked_list.head {
            None => None,
            Some(ref rc) => Some(rc.deref().borrow().value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Queue, LinkedList};

    #[test]
    fn prepend_empty() {
        // Arrange
        let mut linked_list = LinkedList::new();

        // Act
        linked_list.prepend(1);
        linked_list.prepend(2);

        // Assert
        assert_eq!(linked_list.length(), 2);
    }

    #[test]
    fn append_empty() {
        // Arrange
        let mut linked_list = LinkedList::new();

        // Act
        linked_list.append(1);
        linked_list.append(2);

        // Assert
        assert_eq!(linked_list.length(), 2);
    }

    #[test]
    fn enqueue() {
        // Arrange
        let mut queue = Queue::new();

        // Act
        queue.enqueue(1);

        // Assert
        assert_eq!(queue.peek().unwrap(), 1);
    }

    #[test]
    fn lifo() {
        // Arrange
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);

        // Act
        let value = queue.dequeue().unwrap();

        // Assert
        assert_eq!(1, value);
    }
}
