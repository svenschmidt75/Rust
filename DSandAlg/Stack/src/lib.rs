use std::ops::Deref;

struct Node {
    value: u64,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: u64) -> Node {
        Node { value, next: None }
    }

    fn append(&mut self, value: u64) -> &Node {
        let mut leaf = self.find_leaf();
        let node = Box::new(Node::new(value));
        leaf.next = Some(node);
        leaf.next.as_ref().unwrap()
    }

    fn find_leaf(&mut self) -> &mut Node {
        let mut current_node = self;
        while let Some(ref mut next) = current_node.next {
            current_node = next;
        }
        current_node
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
            Some(ref mut node) => node.find_mut(value),
        }
    }

    fn add(&mut self, value: u64) {
        let next = self.next.take();
        let mut node = Box::new(Node::new(value));
        node.next = next;
        self.next = Some(node);
    }

    fn remove(&mut self, value: u64) {
        let mut parent = self;
        while parent.next.is_some() && parent.next.as_ref().unwrap().value != value {
            parent = parent.next.as_mut().unwrap().as_mut();
        }
        if let Some(ref mut child) = parent.next {
            assert_eq!(child.value, value);
            let child_next = child.next.take();
            parent.next = child_next;
        }
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
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn append(&mut self, value: u64) -> &Node {
        match self.head {
            None => {
                let node = Box::new(Node::new(value));
                self.head = Some(node);
                let v1 = self.head.as_mut().unwrap();
                v1
            }
            Some(ref mut node) => node.append(value),
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
            Some(ref mut node) => node.find_mut(value),
        }
    }

    fn remove(&mut self, value: u64) {
        if let Some(ref mut head) = self.head {
            if head.value == value {
                // SS: remove head

                // SS: next node after head
                let next = self.head.take().unwrap().next.take();
                if let Some(n) = next {
                    self.head = Some(n);
                }
            } else {
                head.remove(value);
            }
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

struct Stack {
    linked_list: LinkedList,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            linked_list: LinkedList::new(),
        }
    }

    fn push(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        let head = self.linked_list.head.take();
        node.next = match head {
            None => None,
            Some(node) => Some(node),
        };
        self.linked_list.head = Some(node);
    }

    fn pop(&mut self) -> Option<u64> {
        if self.linked_list.head.is_none() {
            None
        } else {
            let mut head = self.linked_list.head.take().unwrap();
            self.linked_list.head = head.next.take();
            Some(head.value)
        }
    }

    fn length(&self) -> usize {
        self.linked_list.length()
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    #[test]
    fn length() {
        // Arrange

        // Act
        let mut stack = Stack::new();

        // Assert
        assert_eq!(0, stack.length());
    }

    #[test]
    fn push() {
        // Arrange
        let mut stack = Stack::new();

        // Act
        stack.push(1);

        // Assert
        assert_eq!(1, stack.length());
    }

    #[test]
    fn pop_empty() {
        // Arrange
        let mut stack = Stack::new();

        // Act
        let item = stack.pop();

        // Assert
        assert!(item.is_none());
    }

    #[test]
    fn pop() {
        // Arrange
        let mut stack = Stack::new();
        stack.push(1);

        // Act
        let item = stack.pop().unwrap();

        // Assert
        assert_eq!(1, item);
        assert_eq!(0, stack.length());
    }
}
