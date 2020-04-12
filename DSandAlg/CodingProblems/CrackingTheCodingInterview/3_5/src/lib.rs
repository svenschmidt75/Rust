// Cracking the Coding Interview
// 6th ed, p. 99, ex. 3.5

use std::collections::VecDeque;

fn sort_stack(stack: &mut VecDeque<i32>) {
    if stack.is_empty() {
        return;
    }

    let mut tmp_stack = VecDeque::new();
    let item = stack.pop_back().unwrap();
    tmp_stack.push_back(item);

    while stack.is_empty() == false {
        let item = stack.pop_back().unwrap();
        let &other = tmp_stack.back().unwrap();
        if item >= other {
            tmp_stack.push_back(item);
        } else {
            // SS: swap
            let other = tmp_stack.pop_back().unwrap();
            tmp_stack.push_back(item);
            tmp_stack.push_back(other);
        }
    }

    let item = tmp_stack.pop_back().unwrap();
    stack.push_back(item);

    while tmp_stack.is_empty() == false {
        let other = tmp_stack.pop_back().unwrap();
        let &item = stack.back().unwrap();
        if item < other {
            let item = stack.pop_back().unwrap();
            stack.push_back(other);
            stack.push_back(item);
        } else {
            stack.push_back(other);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let mut stack = VecDeque::new();
        stack.push_back(3);
        stack.push_back(7);
        stack.push_back(5);
        stack.push_back(4);
        stack.push_back(1);

        // Act
        sort_stack(&mut stack);

        // Assert
        assert_eq!(1, stack.pop_back().unwrap());
        assert_eq!(3, stack.pop_back().unwrap());
        assert_eq!(4, stack.pop_back().unwrap());
        assert_eq!(5, stack.pop_back().unwrap());
        assert_eq!(7, stack.pop_back().unwrap());
    }
}
