use crate::Stack::Stack;

struct Queue {
    enqueue_stack: Stack,
    dequeue_stack: Stack,
}

impl Queue {

    fn new() -> Queue {
        Queue { enqueue_stack: Stack::new(), dequeue_stack: Stack::new() }
    }

    fn enqueue(&mut self, item: u64) {
        self.enqueue_stack.push(item);
    }

    fn dequeue(&mut self) -> u64 {
        if self.dequeue_stack.is_empty() {
            while self.enqueue_stack.is_empty() == false {
                let item = self.enqueue_stack.pop();
                self.dequeue_stack.push(item);
            }
        }
        self.dequeue_stack.pop()
    }

    // SS: this should not be mut here, something like interior mutability should be used
    // but cannot return a reference to something inside a RefCell...
    fn peek(&mut self) -> Option<&u64> {
        if self.dequeue_stack.is_empty() {
            while self.enqueue_stack.is_empty() == false {
                let item = self.enqueue_stack.pop();
                self.dequeue_stack.push(item);
            }
        }
        self.dequeue_stack.peek()
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
