
fn reverse(input_str: &str) -> String {
    let mut m: Vec<char> = input_str.chars().collect();
    for (i, c) in input_str.chars().enumerate() {
        let index = m.len() - 1 - i;
        m[index] = c;
    }
    m.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn test_reverse() {
        // Arrange
        let string = "Hi, my name is RoboCop!";

        // Act
        let reversed = reverse(&string);

        // Assert
        assert_eq!("!poCoboR si eman ym ,iH", reversed);
    }
}
