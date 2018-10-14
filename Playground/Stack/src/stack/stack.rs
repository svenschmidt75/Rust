#![allow(dead_code)]

pub struct Stack<T> {
    data: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> T {
        if self.data.len() == 0 {
            panic!("pop: Stack is empty")
        }
        let len = self.data.len() - 1;
        let v = self.data.remove(len);
        v
    }

    fn top(&self) -> &T {
        if self.data.len() == 0 {
            panic!("pop: Stack is empty")
        }
        self.data.last().unwrap()
    }
}

#[test]
fn test_push() {
    // Arrange
    let mut stack = Stack::<i32>::new();

    // Act
    stack.push(10);

    // Assert
    assert_eq!(*stack.top(), 10)
}

#[test]
fn test_lifo() {
    // Arrange
    let mut stack = Stack::<i32>::new();

    // Act
    stack.push(10);
    stack.push(11);
    // Assert
    assert_eq!(stack.pop(), 11);
    assert_eq!(stack.pop(), 10)
}

#[test]
#[should_panic]
fn test_pop_panics_when_empty() {
    // Arrange
    let mut stack = Stack::<i32>::new();

    // Act
    stack.pop();
}

#[test]
#[should_panic]
fn test_top_panics_when_empty() {
    // Arrange
    let stack = Stack::<i32>::new();

    // Act
    stack.top();
}
