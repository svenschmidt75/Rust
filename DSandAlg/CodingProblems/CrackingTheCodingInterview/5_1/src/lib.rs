// Cracking the Coding Interview
// 6th ed, p. 115, ex. 5.1

fn insert_m_into_n_1(n: &mut u32, m: u32, i: usize, j: usize) {
    for k in i..=j {
        let bit = get_bit(m, k - i);
        if bit {
            set_bit(n, k);
        } else {
            clear_bit(n, k);
        }
    }
}

fn get_bit(m: u32, pos: usize) -> bool {
    m & (1u32 << pos as u32) > 0
}

fn set_bit(m: &mut u32, pos: usize) {
    *m = *m | (1u32 << pos as u32);
}

fn clear_bit(m: &mut u32, pos: usize) {
    *m = *m & (!(1u32 << pos as u32));
}

fn insert_m_into_n_2(n: &mut u32, m: u32, i: usize, j: usize) {
    // SS: clear bits i through j in n
    let mut mask = 1;
    let mbits = j as u32 - i as u32;
    for p in 0..mbits {
        mask <<= 1;
        mask += 1;
    }
    *n = *n & !(mask << 2);

    // SS: set m's bits in n
    *n = *n | (m << i as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let mut n = 0b10000000000;
        let m = 0b10011;

        // Act
        insert_m_into_n_1(&mut n, m, 2, 6);

        // Assert
        assert_eq!(0b10001001100, n);
    }

    #[test]
    fn test2() {
        // Arrange
        let mut n = 0b10001111100;
        let m = 0b10011;

        // Act
        insert_m_into_n_1(&mut n, m, 2, 6);

        // Assert
        assert_eq!(0b10001001100, n);
    }

    #[test]
    fn test3() {
        // Arrange
        let mut n = 0b10000000000;
        let m = 0b10011;

        // Act
        insert_m_into_n_2(&mut n, m, 2, 6);

        // Assert
        assert_eq!(0b10001001100, n);
    }

    #[test]
    fn test4() {
        // Arrange
        let mut n = 0b10001111100;
        let m = 0b10011;

        // Act
        insert_m_into_n_2(&mut n, m, 2, 6);

        // Assert
        assert_eq!(0b10001001100, n);
    }
}
