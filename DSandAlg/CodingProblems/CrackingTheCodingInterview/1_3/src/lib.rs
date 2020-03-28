// Cracking the Coding Interview
// 6th ed, p. 90, ex. 1.3

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn urlify(input: &mut String, true_length: usize) {
    let mut input_str = input.chars().collect::<Vec<_>>();
    let mut j = input_str.len() - 1;
    for i in 0..true_length {
        let idx = true_length - 1 - i;
        if input_str[idx] == ' ' {
            input_str[j - 2] = '%';
            input_str[j - 1] = '2';
            input_str[j] = '0';
            j -= 3;
        } else {
            input_str[j] = input_str[idx];
            if j > 0 {
                j -= 1;
            }
        }
    }

    input.clear();
    let s = String::from_iter(input_str);
    input.push_str(&s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let mut url = String::from("Mr John Smith    ");

        // Act
        urlify(&mut url, 13);

        // Assert
        assert_eq!("Mr%20John%20Smith", url);
    }
}
