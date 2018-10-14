#![allow(dead_code)]

use stack::stack::Stack;

pub struct Queue<T> {
    stack1: Stack<T>,
    stack2: Stack<T>
}

impl<T: Sized> Queue<T> {

    pub fn new() -> Queue<T> {
        Queue { stack1: Stack::new(), stack2: Stack::new() }
    }

    fn enqueue(&mut self, value: T) {
        self.stack1.push(value);
    }

    // we return T by value (i.e. we relinquish ownership), so
    // T: Sized, as T must have a defined size (like value types,
    // or trait objects)
    fn dequeue(&mut self) -> T {
        if self.is_empty() {
            panic!("dequeue: Queue is empty")
        }
        if self.stack2.is_empty() == false {
            return self.stack2.pop()
        }
        // Pop off elements of stack1 and insert into stack2. That reverses the
        // order of the elements which we need.
        while self.stack1.is_empty() == false {
            let v = self.stack1.pop();
            self.stack2.push(v);
        }
        self.stack2.pop()
    }

    fn len(&self) -> usize {
        self.stack1.len() + self.stack2.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }

}

#[test]
fn test_euqueue() {
    // Arrange
    let mut queue = Queue::<i32>::new();

    // Act
    // Assert
    queue.enqueue(10);
}

#[test]
fn test_dequeue() {
    // Arrange
    let mut queue = Queue::<i32>::new();

    // Act
    queue.enqueue(10);

    // Assert
    assert_eq!(10, queue.dequeue())
}

#[test]
fn test_fifo() {
    // Arrange
    let mut queue = Queue::<i32>::new();

    // Act
    queue.enqueue(10);
    queue.enqueue(11);

    // Assert
    assert_eq!(10, queue.dequeue());
    assert_eq!(11, queue.dequeue())
}

#[test]
fn test_len() {
    // Arrange
    let mut queue = Queue::<i32>::new();

    // Act
    queue.enqueue(10);
    queue.enqueue(11);

    // Assert
    assert_eq!(2, queue.len())
}
