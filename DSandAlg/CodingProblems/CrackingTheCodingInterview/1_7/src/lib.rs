// Cracking the Coding Interview
// 6th ed, p. 91, ex. 1.7

fn rotate(input: &mut [i32], n: usize) {
    // SS: rotate array clock-wise by 90 degree, in-place
    let rows = n / 2;
    for i in 0..rows {
        for j in i..(n - i - 1) {
            let tmp = get(input, n, i, j);

            let value = get(input, n, n - j - 1, i);
            set(input, n, i, j, value);

            let value = get(input, n, n - i - 1, n - j - 1);
            set(input, n, n - j - 1, i, value);

            let value = get(input, n, j, n - i - 1);
            set(input, n, n - i - 1, n - j - 1, value);

            set(input, n, j, n - i - 1, tmp);
        }
    }
}

fn get(input: &[i32], n: usize, row: usize, col: usize) -> i32 {
    let index = row * n + col;
    input[index]
}

fn set(input: &mut [i32], n: usize, row: usize, col: usize, value: i32) {
    let index = row * n + col;
    input[index] = value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let mut input = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ];

        // Act
        rotate(&mut input, 5);

        // Assert
        assert_eq!(
            vec![
                21, 16, 11, 6, 1, 22, 17, 12, 7, 2, 23, 18, 13, 8, 3, 24, 19, 14, 9, 4, 25, 20, 15,
                10, 5
            ],
            input
        );
    }
}
