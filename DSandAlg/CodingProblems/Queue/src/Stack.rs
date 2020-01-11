
pub(crate) struct Stack {
    data: Vec<u64>
}

impl Stack {

    pub(crate) fn new() -> Stack {
        Stack {data: vec![] }
    }

    pub(crate) fn push(&mut self, item: u64) {
        self.data.push(item);
    }

    pub(crate) fn pop(&mut self) -> u64 {
        let item = self.data.pop().unwrap();
        item
    }

    pub(crate) fn peek(&self) -> Option<&u64> {
        self.data.last()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use crate::Stack::Stack;

    #[test]
    fn push() {
        // Arrange
        let mut stack = Stack::new();

        // Act
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Assert
        assert_eq!(stack.peek().unwrap(), &3);
    }

    #[test]
    fn pop() {
        // Arrange
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Act
        let item = stack.pop();

        // Assert
        assert_eq!(item, 3);
        assert_eq!(stack.peek().unwrap(), &2);
    }


}
