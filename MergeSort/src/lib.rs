// SS: make generic
fn merge(data: &mut [f64], s1: usize, s2: usize, s3: usize) {
    // SS: data[s1..=s2] is sorted, and so is data[s2+1..=s3]
    // Merge them such that the result is sorted as well.
    let mut i = s1;
    let mut j = s2 + 1;

    loop {
        println!("i {} - j {}", i, j);

        if i == s3 {
            // done
            println!("done");
            break;
        }

        if j > s3 {
            println!("j exhausted, advancing i, resetting j");
            i += 1;
            j = if s2 + 1 < i { i + 1 } else { s2 + 1};
            continue;
        }

        if data[i] > data[j] {
            println!("swapping data[{}] ({}) with data[{}] with {}", i, data[i], j, data[j]);
            let tmp = data[i];
            data[i] = data[j];
            data[j] = tmp;
            j += 1;
            continue;
        }

        // SS: data[i] <= data[j]
        j += 1;
    }
}

fn merge_sort(data: &mut [f64]) {
    let mid = data.len() / 2;
    if mid > 1 {
        merge_sort_internal(data, 0, mid - 1);
        merge_sort_internal(data, mid, data.len() - 1);
        merge(data, 0, mid - 1, data.len() - 1);
    }
}

fn merge_sort_internal(data: &mut [f64], low: usize, high: usize) {
    // SS: bounds are inclusive
    let mid = (high - low + 1) / 2;
    if mid > 1 {
        merge_sort_internal(data, low, low + mid - 1);
        merge_sort_internal(data, low + mid, high);
        merge(data, low, low + mid - 1, high);
    }
}

#[cfg(test)]
mod tests {
    use crate::{merge, merge_sort};
    use rand::Rng;

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

    #[test]
    fn test4() {
        // Arrange
        let mut data = [38.0, 27.0, 43.0, 3.0, 9.0, 82.0, 10.0];

        // Act
        merge_sort(&mut data[..]);

        // Assert
        assert_eq!(3.0, data[0]);
        assert_eq!(9.0, data[1]);
        assert_eq!(10.0, data[2]);
        assert_eq!(27.0, data[3]);
        assert_eq!(38.0, data[4]);
        assert_eq!(43.0, data[5]);
        assert_eq!(82.0, data[6]);
    }

    #[test]
    fn test5() {
        // Arrange
        let mut rng = rand::thread_rng();
        let mut data = (1..100).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();

        // Act
        merge_sort(&mut data[..]);

        // Assert
        let t = data.iter().zip(data.iter().skip(1)).fold(true, |accum, (x, y)| accum && *x < *y);
        assert_eq!(t, true);
    }
}
