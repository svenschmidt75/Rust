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

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // prepend is O(1)
    fn prepend(&mut self, value: u64) {
        let mut node = Box::new(Node::new(value));
        match self.head.take() {
            None => {}
            Some(link) => {
                node.next = Some(link);
            }
        }
        self.head = Some(node);
    }

    //    fn remove(&mut self, value: u64) {
    //        if let Some(ref mut head) = self.head {
    //            if head.value == value {
    //                // SS: remove head
    //
    //                // SS: next node after head
    //                let next = self.head.take().unwrap().next.take();
    //                if let Some(n) = next {
    //                    self.head = Some(*n);
    //                }
    //            } else {
    //                head.remove(value);
    //            }
    //        }
    //    }
    //
    //    fn find(&self, value: u64) -> Option<&Node> {
    //        match self.head {
    //            None => None,
    //            Some(ref node) => node.find(value),
    //        }
    //    }
    //
    //    fn find_mut(&mut self, value: u64) -> Option<&mut Node> {
    //        match self.head {
    //            None => None,
    //            Some(ref mut node) => node.find_mut(value),
    //        }
    //    }

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
        linked_list.prepend(2);
        linked_list.prepend(4);
        linked_list.prepend(6);

        // Assert
        //        linked_list.print();
        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn find_success() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        //        let node = linked_list.find(2);

        // Assert
        //        assert!(node.is_some());
    }

    #[test]
    fn find_fail() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        //        let node = linked_list.find(3);

        // Assert
        //        assert!(node.is_none());
    }

    #[test]
    fn add() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        //        let node = linked_list.find_mut(2).unwrap();

        // Act
        //        node.add(8);

        // Assert
        //        linked_list.print();
    }

    #[test]
    fn add_tail() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        //        let node = linked_list.find_mut(6).unwrap();

        // Act
        //        node.add(8);

        // Assert
        //        linked_list.print();
    }

    #[test]
    fn remove_head() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);

        // Act
        //        linked_list.remove(2);

        // Assert
        //        linked_list.print();
    }

    #[test]
    fn remove() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);
        linked_list.prepend(1);

        // Act
        //        linked_list.remove(6);

        // Assert
        //        linked_list.print();
    }

    #[test]
    fn remove_tail() {
        // Arrange
        let mut linked_list = LinkedList::new();
        linked_list.prepend(2);
        linked_list.prepend(6);
        linked_list.prepend(1);

        // Act
        //        linked_list.remove(1);

        // Assert
        //        linked_list.print();
    }
}
