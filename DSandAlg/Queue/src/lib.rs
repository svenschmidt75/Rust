use std::ops::Deref;
use std::rc::Rc;

struct Node {
    value: u64,
    next: Option<Rc<Box<Node>>>,
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
                let rc = current_node.next.as_mut().unwrap();
                let n2 = Rc::get_mut(rc).unwrap().as_mut();
                current_node = n2;
            }
        }
    }

    fn find(&self, value: u64) -> Option<&Node> {
        if self.value == value {
            return Some(self);
        }
        match self.next {
            None => None,
            Some(ref node) => node.find(value),
        }
    }

    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
        if self.value == value {
            return Some(self);
        }
        match self.next {
            None => None,
            Some(ref mut node) => Rc::get_mut(node).unwrap().as_mut().find_mut(value),
        }
    }

    fn remove(&mut self, value: u64) {
        let mut parent = self;
        while parent.next.is_some() && parent.next.as_ref().unwrap().value != value {
            parent = Rc::get_mut(parent.next.as_mut().unwrap()).unwrap().as_mut();
        }
        if let Some(ref mut child) = parent.next {
            assert_eq!(child.value, value);
            let child_next = Rc::get_mut(child).unwrap().next.take();
            parent.next = child_next;
        }
    }

    fn append(&mut self, value: u64) -> Rc<Box<Node>> {
        let mut leaf = self.find_leaf_mut();
        let node = Rc::new(Box::new(Node::new(value)));
        leaf.next = Some(node.clone());
        node.clone()
    }

    fn print(&self) {
        print!("{} ", self.value);
        match self.next {
            None => {}
            Some(ref next) => {
                next.print();
            }
        }
    }
}

struct LinkedList {
    head: Option<Rc<Box<Node>>>,
    tail: Option<Rc<Box<Node>>>,
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
                let node = Rc::new(Box::new(Node::new(value)));
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
            Some(ref mut node) => {
                let new_tail = Rc::get_mut(node).unwrap().as_mut().append(value);
                self.tail = Some(new_tail);
            }
        }
    }

    fn prepend(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        let head = self.head.take();
        node.next = match head {
            None => None,
            Some(node) => Some(node),
        };
        self.head = Some(Rc::new(node));
    }

    fn remove(&mut self, value: u64) {
        if let Some(ref mut head) = self.head {
            if head.value == value {
                // SS: remove head

                // SS: next node after head
                let next = Rc::get_mut(self.head.take().as_mut().unwrap())
                    .unwrap()
                    .next
                    .take();
                if let Some(n) = next {
                    self.head = Some(n);
                }
            } else {
                Rc::get_mut(head).unwrap().remove(value);
            }
        }
    }

    fn find(&self, value: u64) -> Option<&Node> {
        match self.head {
            None => None,
            Some(ref node) => node.find(value),
        }
    }

    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
        match self.head {
            None => None,
            Some(ref mut node) => Rc::get_mut(node).unwrap().find_mut(value),
        }
    }

    fn length(&self) -> usize {
        if let None = self.head {
            0
        } else {
            let mut count = 1;
            let mut node = self.head.as_ref().unwrap();
            while node.next.is_some() {
                count = count + 1;
                node = node.next.as_ref().unwrap();
            }
            count
        }
    }

    fn print(&self) {
        match self.head {
            None => println!("empty"),
            Some(ref node) => {
                node.print();
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
        self.linked_list.prepend(value);
    }

    fn dequeue(&mut self) -> Option<u64> {
        let mut head = self.linked_list.head.take();
        match head {
            None => None,
            Some(ref mut rc) => {
                // SS: set new head to next node
                self.linked_list.head = rc.next.clone();
                Some(rc.value)
            }
        }
    }

    fn peek(&self) -> Option<u64> {
        match self.linked_list.head {
            None => None,
            Some(ref rc) => Some(rc.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Queue;

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
