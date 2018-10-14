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
        if self.empty() {
            panic!("pop: Stack is empty")
        }

    }

    fn len(&self) -> usize {
        self.stack1.len() + self.stack2.len()
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }

}