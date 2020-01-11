


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
