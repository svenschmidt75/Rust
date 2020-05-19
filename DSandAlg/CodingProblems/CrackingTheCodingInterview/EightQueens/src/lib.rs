// SS: solve 8 queens problem
// https://en.wikipedia.org/wiki/Eight_queens_puzzle
// SS: We are calculating all 92 solutions, not just the fundamental ones...

use std::cmp;

fn bit_set(index: usize, bits: u16) -> bool {
    let bit = 1 << index;
    let bit_set = bits & bit;
    bit_set > 0
}

fn set_bit(index: usize, bits: &mut u16) {
    let bit = 1 << index;
    *bits = *bits | bit;
}

fn get_left_diag_index(row: u8, col: u8) -> i8 {
    // SS: left diagonal index
    const OFFSET: i8 = 8 - 1;
    let diagonal_index = row as i8 - col as i8;
    diagonal_index + OFFSET
}

fn get_right_diag_index(row: u8, col: u8) -> i8 {
    // SS: right diagonal index
    const OFFSET: i8 = 8 - 1;
    const NCOLS: i8 = 8;
    let diagonal_index = row as i8 - (NCOLS - 1 - col as i8);
    diagonal_index + OFFSET
}

fn solve() -> u32 {
    let n = solve_recursive(7, 0, 0, 0);
    n
}

fn solve_recursive(row: i8, columns: u8, left_diag: u16, right_diag: u16) -> u32 {
    // SS: greedy algorithm
    // SS: runtime: each row, check each column
    // Since each column, left and right diagonal check is O(1), we find O(nrows * ncols) = O(8^8)

    if row < 0 {
        1
    } else {
        let mut n = 0;
        for col in 0..8 {
            // SS: check if column can be used
            if bit_set(col, columns as u16) {
                // SS: there is already a queen on that column
                continue;
            }

            // SS: check left diagonal
            let left_diag_index = get_left_diag_index(row as u8, col as u8);
            if (bit_set(left_diag_index as usize, left_diag)) {
                // SS: there is already a queen on the left diagonal
                continue;
            }

            // SS: check right diagonal
            let right_diag_index = get_right_diag_index(row as u8, col as u8);
            if (bit_set(right_diag_index as usize, right_diag)) {
                // SS: there is already a queen on the right diagonal
                continue;
            }

            // SS: place queen here at (row, col)
            let mut c = columns as u16;
            set_bit(col, &mut c);

            let mut ld = left_diag;
            set_bit(left_diag_index as usize, &mut ld);

            let mut rd = right_diag;
            set_bit(right_diag_index as usize, &mut rd);

            let ns = solve_recursive(row - 1, c as u8, ld, rd);
            n += ns;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Arrange

        // Act
        let n = solve();

        // Assert
        assert_eq!(n, 92);
    }

    #[test]
    fn test_left_diagonal_1() {
        // Arrange
        let (row, col) = (0, 7);

        // Act
        let left_diag_index = get_left_diag_index(row, col);

        // Assert
        assert_eq!(left_diag_index, 0);
    }

    #[test]
    fn test_left_diagonal_2() {
        // Arrange
        let (row, col) = (7, 0);

        // Act
        let left_diag_index = get_left_diag_index(row, col);

        // Assert
        assert_eq!(left_diag_index, 14);
    }

    #[test]
    fn test_right_diagonal_1() {
        // Arrange
        let (row, col) = (0, 0);

        // Act
        let left_diag_index = get_right_diag_index(row, col);

        // Assert
        assert_eq!(left_diag_index, 0);
    }

    #[test]
    fn test_right_diagonal_2() {
        // Arrange
        let (row, col) = (7, 7);

        // Act
        let left_diag_index = get_right_diag_index(row, col);

        // Assert
        assert_eq!(left_diag_index, 14);
    }
}
