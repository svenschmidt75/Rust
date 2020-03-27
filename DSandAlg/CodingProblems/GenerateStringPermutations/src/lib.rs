// Cracking the Coding Interview
// 6th ed, page 71

fn generate_permutations(input: &str) -> Vec<String> {
    // SS: Given a string, generate all possible permutations
    let results = generate_permutations_internal(input);
    results
}

fn generate_permutations_internal(input: &str) -> Vec<String> {
    let n = input.len();
    if n == 1 {
        // SS: base case
        vec![input.to_owned()]
    } else {
        // SS: generate all permutations of length n - 1
        let results = generate_permutations_internal(&input[1..]);
        let mut local_results = vec![];
        let c = input.chars().next().unwrap();
        for s in results {
            for i in 0..=s.len() {
                let mut tmp = s[0..i].to_owned();
                tmp.push(c);
                tmp.push_str(&s[i..]);
                local_results.push(tmp);
            }
        }
        local_results
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_permutations;

    #[test]
    fn test1() {
        // Arrange
        let input = "a";

        // Act
        let result = generate_permutations(&input);

        // Assert
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = "ab";

        // Act
        let result = generate_permutations(&input);

        // Assert
        assert_eq!(result, vec!["ab", "ba"]);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = "abc";

        // Act
        let result = generate_permutations(&input);

        // Assert
        assert_eq!(result, vec!["abc", "bac", "bca", "acb", "cab", "cba"]);
    }

    #[test]
    fn test4() {
        // Arrange
        let input = "abcdef";

        // Act
        let result = generate_permutations(&input);

        // Assert
        assert_eq!(result.len(), 1 * 2 * 3 * 4 * 5 * 6);
    }
}
