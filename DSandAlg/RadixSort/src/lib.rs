use std::cmp;

fn get_digit(num: i64, pos: usize) -> i64 {
    let p1 = 10_i64.pow(pos as u32);

    // SS: input: num=94671, pos=2
    // SS: 94671 / 100 = 946
    let t1 = num / p1;

    // SS: 946 / 10 = 94.6, or 946 % 10 = 6
    let digit = t1.abs() % 10;
    digit.abs()
}

fn number_of_digits(num: i64) -> usize {
    if num == 0 {
        1
    } else {
        let num_log = (num.abs() as f64).log10() as usize;
        num_log + 1
    }
}

fn largest_number_of_digits(input: &[i64]) -> usize {
    if input.is_empty() {
        0
    } else {
        let mut max = 0;
        for i in 0..input.len() {
            let digits = number_of_digits(input[i]);
            max = cmp::max(max, digits);
        }
        max
    }
}

fn radix_sort(input: &mut [i64]) {
    // O(N * k) sort, k = largest number of digits in numbers
    // space complexity: N items in bucket
    let n_iterations = largest_number_of_digits(&input);

    let mut buckets: [Vec<i64>; 10] = [
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
        Vec::with_capacity(10),
    ];

    // O(k) outer loop, k = max. digits
    for k in 0..n_iterations {
        // O(N) inner loop
        for n in 0..input.len() {
            let value = input[n];
            let digit = get_digit(value, k);
            let bucket = &mut buckets[digit as usize];
            bucket.push(value);
        }

        // SS: restore array from buckets, reordering
        let mut cnt = 0;

        // SS: flatten out the buckets, O(1)
        let tmp = buckets
            .iter()
            .flat_map(|x| x)
            .map(|&x| x)
            .collect::<Vec<_>>();
        input.copy_from_slice(&tmp);

        // O(1)
        buckets.iter_mut().for_each(|x| x.clear());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort_1() {
        // Arrange
        let mut number = [23, 567, 89, 12234324, 90];

        // Act
        radix_sort(&mut number[..]);

        // Assert
        assert_eq!(&number, &[23, 89, 90, 567, 12234324]);
    }

    #[test]
    fn test_radix_sort_2() {
        // Arrange
        let mut number = [
            3221, 1, 10, 9680, 577, 9420, 7, 5622, 4793, 2030, 3138, 82, 2599, 743, 4127,
        ];

        // Act
        radix_sort(&mut number[..]);

        // Assert
        assert_eq!(
            &number,
            &[1, 7, 10, 82, 577, 743, 2030, 2599, 3138, 3221, 4127, 4793, 5622, 9420, 9680]
        );
    }

    #[test]
    fn test_largest_number_of_digits() {
        // Arrange
        let number = [23, 567, 89, 12234324, 90];

        // Act
        let result = largest_number_of_digits(&number);

        // Assert
        assert_eq!(result, 8);
    }

    #[test]
    fn test_number_of_digits1() {
        // Arrange
        let number = -94671;

        // Act
        let digits = number_of_digits(number);

        // Assert
        assert_eq!(5, digits);
    }

    #[test]
    fn test_get_digit_negative() {
        // Arrange
        let number = -94671;

        // Act
        let digit = get_digit(number, 2);

        // Assert
        assert_eq!(6, digit);
    }

    #[test]
    fn test_get_digit1() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 2);

        // Assert
        assert_eq!(6, digit);
    }

    #[test]
    fn test_get_digit2() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 0);

        // Assert
        assert_eq!(1, digit);
    }

    #[test]
    fn test_get_digit3() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 3);

        // Assert
        assert_eq!(4, digit);
    }

    #[test]
    fn test_get_digit4() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 4);

        // Assert
        assert_eq!(9, digit);
    }

    #[test]
    fn test_get_digit5() {
        // Arrange
        let number = 94671;

        // Act
        let digit = get_digit(number, 5);

        // Assert
        assert_eq!(0, digit);
    }
}
