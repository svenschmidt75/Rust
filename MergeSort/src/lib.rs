// SS: make generic
fn merge(data: &mut [f64], s1: usize, s2: usize, s3: usize) {
    // SS: data[s1..=s2] is sorted, and so is data[s2+1..=s3]
    // Merge them such that the result is sorted as well.
    let mut i = s1;
    let mut j = s2 + 1;

    while i < s3 {
        if data[i] > data[j] {
            let tmp = data[i];
            data[i] = data[j];
            data[j] = tmp;
            i += 1;
            continue;
        }

        if i == j {
            if i == s3 {
                // SS: done, all is sorted
                break;
            }
            j += 1;
            continue;
        }

        if data[i] <= data[j] {
            j += 1;
            continue;
        }

    }
}


#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn test1() {
        // Arrange
        let mut data = [27.0, 38.0, 3.0, 43.0];

        // Act
        merge(&mut data[..], 0, 1, 3);

        // Assert
        assert_eq!(3.0, data[0]);
        assert_eq!(27.0, data[1]);
        assert_eq!(38.0, data[2]);
        assert_eq!(43.0, data[3]);
    }

    #[test]
    fn test2() {
        // Arrange
        let mut data = [27.0, 49.0, 3.0, 43.0];

        // Act
        merge(&mut data[..], 0, 1, 3);

        // Assert
        assert_eq!(3.0, data[0]);
        assert_eq!(27.0, data[1]);
        assert_eq!(43.0, data[2]);
        assert_eq!(49.0, data[3]);
    }

    #[test]
    fn test3() {
        // Arrange
        let mut data = [3.0, 27.0, 38.0, 43.0, 9.0, 10.0, 82.0];

        // Act
        merge(&mut data[..], 0, 3, 6);

        // Assert
        assert_eq!(3.0, data[0]);
        assert_eq!(9.0, data[1]);
        assert_eq!(10.0, data[2]);
        assert_eq!(27.0, data[3]);
        assert_eq!(38.0, data[4]);
        assert_eq!(43.0, data[5]);
        assert_eq!(82.0, data[6]);
    }
}
