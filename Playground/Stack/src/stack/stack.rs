#![allow(dead_code)]

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> T {
        if self.data.len() == 0 {
            panic!("pop: Stack is empty")
        }
        let len = self.data.len() - 1;
        let v = self.data.remove(len);
        v
    }

    pub fn top(&self) -> &T {
        if self.data.len() == 0 {
            panic!("pop: Stack is empty")
        }
        self.data.last().unwrap()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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

#[test]
fn test_palindrome() {
    // Arrange
    let input = "aabaabaXabaabaa";
    let pair = input.split('X').collect::<Vec<&str>>();
    let part1 = pair[0];
    let part2 = pair[1];
    let mut stack = Stack::<char>::new();
    for c in part1.chars() {
        stack.push(c);
    }

    // Act
    // Assert
    assert_eq!(part1.len(), part2.len());

    for c2 in part2.chars() {
        let c1 = stack.pop();
        assert_eq!(c1, c2);
    }
}

#[test]
fn test_reverse_elements() {
    // Arrange
    let input = vec![1, 2, 3];

    let mut input_reversed_expected = input.clone();
    input_reversed_expected.reverse();

    let mut stack1 = Stack::<i32>::new();
    for c in &input {
        stack1.push(*c);
    }
    let mut input_reversed = Vec::<i32>::new();

    // Act
    while stack1.is_empty() == false {
        let v = stack1.pop();
        input_reversed.push(v);
    }

    // Assert
    assert_eq!(input_reversed_expected, input_reversed)
}
