use std::mem;

fn reverse(input_str: &str) -> String {
    let mut m: Vec<char> = input_str.chars().collect();
    for i in 0..(m.len() / 2) {
        let idx = m.len() - 1 - i;
        swap(&mut m, i, idx);
    }
    m.into_iter().collect()
}

fn swap(array: &mut [char], a: usize, b: usize) {
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
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
