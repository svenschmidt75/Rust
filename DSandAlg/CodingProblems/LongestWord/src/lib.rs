fn longest_word(input: &str) -> &str {
    // SS: check for length of string, must fit in i32
    let mut length = 0;
    let mut start = input.len();
    let mut longest_length = 0;
    let mut longest_start = input.len();

    for (i, c) in input.chars().enumerate() {
        if c.is_alphabetic() {
            length += 1;
            if start == input.len() {
                start = i;
            }
        } else {
            // SS: restart
            if length > longest_length || longest_length == input.len() {
                longest_length = length;
                longest_start = start;
            }
            length = 0;
            start = input.len();
        }
    }

    if length > longest_length || longest_length == input.len() {
        longest_length = length;
        longest_start = start;
    }

    let end_pos = longest_start + longest_length;
    &input[longest_start..end_pos]
}

#[cfg(test)]
mod tests {
    use crate::longest_word;

    #[test]
    fn test1() {
        // Arrange
        let input = "fun&!! time";

        // Act
        let word = longest_word(input);

        // Assert
        assert_eq!("time", word);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = "I love dogs";

        // Act
        let word = longest_word(input);

        // Assert
        assert_eq!("love", word);
    }
}
