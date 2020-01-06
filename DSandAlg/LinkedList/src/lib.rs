use std::mem;

type Link = Option<Box<Node>>;

struct LinkedList {
    head: Link,
}

struct Node {
    value: u64,
    next: Link,
}

impl Node {
    fn new(value: u64) -> Node {
        Node { value, next: None }
    }

    // insert
    // Insert a new node after the current node.
    // Runtime complexity is O(1)
    fn insert(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        node.next = self.next.take();
        self.next = Some(node);
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

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // prepend
    // Prepend a new node at the current head.
    // Runtime complexity is O(1)
    fn prepend(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        node.next = self.head.take();
        self.head = Some(node);
    }

    // append
    // Append a new node to the tail of the linked list. Finding the tails has
    // O(N) runtime complexity.
    fn append(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        if self.head.is_none() {
            self.head = Some(node);
        } else {
            // SS: find tail
            let mut link = self.head.as_mut().unwrap();
            while let Some(ref mut next) = link.next {
                link = next;
            }

            let mut node = Box::new(Node::new(value));
            link.next = Some(node);
        }
    }

    fn find_parent(&mut self, value: u64) -> Option<&mut Node> {
        let mut child = self.head.as_mut().unwrap();
        loop {
            if child.next.is_none() {
                return None;
            }
            if child.next.as_ref().unwrap().value == value {
                return Some(child);
            }
            child = child.next.as_mut().unwrap();
        }
    }

    fn remove(&mut self, value: u64) {
        if let Some(_) = self.head {
            if self.head.as_ref().unwrap().value == value {
                // SS: remove head
                self.head.take();
            } else {
                // SS: find parent of node to delete
                let mut parent = self.find_parent(value);
                if let Some(ref mut p) = parent {
                    let mut child = p.next.take().unwrap();
                    let next = child.next.take();
                    p.next = next;
                }
            }
        }
    }

    fn find(&self, value: u64) -> Option<&Node> {
        let mut link = &self.head;
        while let Some(ref next) = link {
            if next.value == value {
                return Some(next);
            }
            link = &next.next;
        }
        None
    }

    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
        let mut link = &mut self.head;
        while let Some(ref mut next) = link {
            if next.value == value {
                return Some(next);
            }
            link = &mut next.next;
        }
        None
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

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn prepend() {
        // Arrange
        let mut linked_list = LinkedList::new();

        // Act
        linked_list.prepend(2);
        linked_list.prepend(4);
        linked_list.prepend(6);

        // Assert
        linked_list.print();
        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn append() {
        // Arrange
        let mut linked_list = LinkedList::new();

        // Act
        linked_list.append(2);
        linked_list.append(4);
        linked_list.append(6);

        // Assert
        linked_list.print();
        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn find_success() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        let node = linked_list.find(2);

        // Assert
        assert!(node.is_some());
    }

    #[test]
    fn find_fail() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        let node = linked_list.find(3);

        // Assert
        assert!(node.is_none());
    }

    #[test]
    fn add() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);
        let node = linked_list.find_mut(2).unwrap();

        // Act
        node.insert(8);

        // Assert
        linked_list.print();
    }

    #[test]
    fn remove_head() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        linked_list.remove(2);

        // Assert
        linked_list.print();
    }

    #[test]
    fn remove() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);
        linked_list.prepend(1);

        // Act
        linked_list.remove(6);

        // Assert
        linked_list.print();
    }

    #[test]
    fn remove_tail() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);
        linked_list.prepend(1);

        // Act
        linked_list.remove(2);

        // Assert
        linked_list.print();
    }
}
