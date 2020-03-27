// https://practice.geeksforgeeks.org/problems/left-view-of-binary-tree/1

fn generate_left_view<'a>(input: &[&'a str]) -> Vec<&'a str> {
    // SS: Input is a character sequence generated from a binary tree using
    // level-order traversal. N indicates a null-child.
    // We are not explicitly constructing a binary tree, as I don't think
    // it is needed for the left view. For the right view, it is though,
    // because not each level is filled out entirely...

    let mut result = vec![];

    let mut parent = 0;
    let mut child = 0;
    while parent < input.len() {
        let p = input[parent];
        result.push(p);

        // SS: index of left child
        child = 2 * parent + 1;
        if child >= input.len() {
            break;
        } else if input[child] == "N" {
            // SS: no left child, right child present?
            if input[child + 1] == "N" {
                break;
            } else {
                parent = child + 1;
            }
        } else {
            parent = child;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::generate_left_view;

    #[test]
    fn test1() {
        // Arrange
        let input = [
            "1", "2", "3", "N", "N", "4", "6", "N", "5", "N", "N", "7", "N",
        ];

        // Act
        let path = generate_left_view(&input[..]);

        // Assert
        assert_eq!(path, vec!["1", "2"]);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = ["10", "20", "30", "40", "60", "N", "N"];

        // Act
        let path = generate_left_view(&input[..]);

        // Assert
        assert_eq!(path, vec!["10", "20", "40"]);
    }

    #[test]
    fn test3() {
        // Arrange
        let input = [
            "1", "2", "3", "4", "5", "6", "7", "N", "8", "N", "N", "N", "N", "N", "N", "N", "N",
        ];

        // Act
        let path = generate_left_view(&input[..]);

        // Assert
        assert_eq!(path, vec!["1", "2", "4", "8"]);
    }
}
