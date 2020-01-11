use crate::Stack::Stack;

struct Queue {
    stack1: Stack,
    stack2: Stack,
}

impl Queue {

    fn new() -> Queue {
        Queue {stack1: Stack::new(), stack2: Stack::new() }
    }

    fn enqueue(&mut self, item: u64) {
        self.stack1.push(item);
    }

    fn dequeue(&mut self) -> u64 {
        if self.stack2.is_empty() {
            while self.stack1.is_empty() == false {
                let item = self.stack1.pop();
                self.stack2.push(item);
            }
        }
        self.stack2.pop()
    }

    // SS: this should not be mut here, something like interior mutability should be used
    // but cannot return a reference to something inside a RefCell...
    fn peek(&mut self) -> Option<&u64> {
        if self.stack2.is_empty() {
            while self.stack1.is_empty() == false {
                let item = self.stack1.pop();
                self.stack2.push(item);
            }
        }
        self.stack2.peek()
    }
}


#[cfg(test)]
mod tests {
    use crate::QueueWithStacks::Queue;

    #[test]
    fn enqueue() {
        // Arrange
        let mut queue = Queue::new();

        // Act
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        // Assert
        assert_eq!(queue.peek().unwrap(), &1);
    }

    #[test]
    fn dequeue() {
        // Arrange
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        // Act
        let item = queue.dequeue();

        // Assert
        assert_eq!(item, 1);
        assert_eq!(queue.peek().unwrap(), &2);
    }

}
