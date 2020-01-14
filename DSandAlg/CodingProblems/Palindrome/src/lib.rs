fn find_palindromes(input: &str) -> Vec<&str> {
    let mut result = vec![];
    let array = input.bytes().collect::<Vec<_>>();
    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            if array[i] == array[j] && is_word_begin(input, i) && is_word_end(input, j) {
                let mut cnt = 1;
                while i + cnt < j - cnt && array[i + cnt] == array[j - cnt] {
                    cnt += 1;
                }
                let is_event = (j - i + 1) % 2 == 0;
                if is_event && i == j + 1 || is_event == false && i + cnt == j - cnt {
                    // SS: found palindrome
                    result.push(&input[i..=j]);
                }
            }
        }
    }
    result
}

fn is_word_begin(input: &str, index: usize) -> bool {
    index == 0 || input.chars().nth(index - 1).unwrap().is_alphabetic() == false
}

fn is_word_end(input: &str, index: usize) -> bool {
    index == input.len() - 1 || input.chars().nth(index + 1).unwrap().is_alphabetic() == false
}

#[cfg(test)]
mod tests {
    use crate::find_palindromes;

    #[test]
    fn palindrome1() {
        // Arrange
        let input = "madam";

        // Act
        let results = find_palindromes(input);

        // Act
        assert_eq!("madam", results[0]);
    }

    #[test]
    fn palindrome2() {
        // Arrange
        let input = "madfam";

        // Act
        let results = find_palindromes(input);

        // Act
        assert_eq!(0, results.len());
    }

    #[test]
    fn palindrome3() {
        // Arrange
        let input = "What kind of madam are you?";

        // Act
        let results = find_palindromes(input);

        // Act
        assert_eq!("madam", results[0]);
    }
}
