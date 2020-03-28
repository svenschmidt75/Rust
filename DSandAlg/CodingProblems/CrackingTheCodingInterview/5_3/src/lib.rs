// Cracking the Coding Interview
// 6th ed, p. 116, ex. 5.3

use std::cmp;

fn longest(input: u32) -> usize {
    let partition = partition(input);

    // SS: try finding the max. number of 1s by merging
    // 2 adjacent partitions
    // runtime O(mbits)
    let mut max = 0;
    for i in 0..partition.len() {
        let p1 = partition[i];
        let mut cmax = p1.1 - p1.0 + 2;
        if i < partition.len() - 1 {
            let p2 = partition[i + 1];
            if p1.1 + 2 == p2.0 {
                cmax += p2.1 - p2.0 + 1;
            }
        }
        max = cmp::max(max, cmax);
    }

    max
}

fn partition(input: u32) -> Vec<(usize, usize)> {
    // SS: partition input into groupings of 1s
    // runtime O(mbits), O(mbits) memory
    let mbits = (input as f64).log2() as usize;
    let mut partition = vec![];
    let mut start = 0;
    let mut current = 0;
    let mut state = false;
    while current <= mbits {
        let bit = get_bit(input, current);
        if state == false && bit {
            start = current;
            state = true;
        } else if state && bit == false {
            partition.push((start, current - 1));
            state = false;
        }
        current += 1;
    }
    if state {
        partition.push((start, mbits));
    }
    partition
}

fn get_bit(m: u32, pos: usize) -> bool {
    m & (1u32 << pos as u32) > 0
}

fn longest2(input: u32) -> usize {
    // SS: runtime O(bits), O(1) memory
    let mbits = (input as f64).log2() as usize;
    let mut start = 0;
    let mut state = false;
    let mut zero = 0;
    let mut nzero = 0;
    let mut max = 0;
    for current in 0..=mbits {
        let bit = get_bit(input, current);
        if state == false && bit {
            start = current;
            state = true;
        } else if state && bit == false {
            nzero += 1;
            if nzero == 2 {
                max = cmp::max(max, current - start);
                nzero = 1;
                start = zero + 1;
            }
            // SS: remember the position of the last 0
            zero = current;
        }
    }
    if state {
        max = cmp::max(max, mbits - start + 1);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let num = 0b11011101111;

        // Act
        let max = longest(num);

        // Assert
        assert_eq!(max, 8);
    }

    #[test]
    fn test12() {
        // Arrange
        let num = 0b110111001111;

        // Act
        let max = longest(num);

        // Assert
        assert_eq!(max, 6);
    }

    #[test]
    fn test13() {
        // Arrange
        let num = 0b1111101101111;

        // Act
        let max = longest(num);

        // Assert
        assert_eq!(max, 8);
    }

    #[test]
    fn test21() {
        // Arrange
        let num = 0b11011101111;

        // Act
        let max = longest2(num);

        // Assert
        assert_eq!(max, 8);
    }

    #[test]
    fn test22() {
        // Arrange
        let num = 0b110111001111;

        // Act
        let max = longest2(num);

        // Assert
        assert_eq!(max, 6);
    }

    #[test]
    fn test23() {
        // Arrange
        let num = 0b1111101101111;

        // Act
        let max = longest2(num);

        // Assert
        assert_eq!(max, 8);
    }
}
