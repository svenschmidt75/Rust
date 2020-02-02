// https://www.reddit.com/r/AlgoExpert/comments/ev66aa/day_6_20200128_problem_of_the_day_asked_by_amazon/

fn flat_index(row: i64, col: i64, ncols: usize) -> usize {
    (row * ncols as i64 + col) as usize
}

fn matrix_spiral(matrix: &[i64], nrows: usize, ncols: usize) -> Vec<i64> {
    let mut start_row: i64 = 0;
    let mut end_row: i64 = nrows as i64 - 1;
    let mut start_col: i64 = 0;
    let mut end_col: i64 = ncols as i64 - 1;

    let mut row: i64 = 0;
    let mut col: i64 = 0;

    let mut result = vec![];

    while start_row < end_row && start_col < end_col {
        // SS: stage 1, move in positive col direction
        while col <= end_col {
            let index = flat_index(row, col, ncols);
            let value = matrix[index];
            result.push(value);
            col += 1;
        }

        start_row += 1;
        row += 1;
        col = end_col;

        // SS: stage 2, move in positive row direction
        while row <= end_row {
            let index = flat_index(row, col, ncols);
            let value = matrix[index];
            result.push(value);
            row += 1;
        }

        end_col -= 1;
        col -= 1;
        row = end_row;

        // SS: stage 3, move in negative col direction
        while col >= start_col {
            let index = flat_index(row, col, ncols);
            let value = matrix[index];
            result.push(value);
            col -= 1;
        }

        end_row -= 1;
        row -= 1;
        col = start_col;

        // SS: stage 4, move in negative row direction
        while row >= start_row {
            let index = flat_index(row, col, ncols);
            let value = matrix[index];
            result.push(value);
            row -= 1;
        }

        start_col += 1;
        row += 1;
        col = start_col;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::matrix_spiral;

    #[test]
    fn test1() {
        // Arrange
        let matrix = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        // Act
        let result = matrix_spiral(&matrix, 4, 5);

        // Assert
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5, 10, 15, 20, 19, 18, 17, 16, 11, 6, 7, 8, 9, 14, 13, 12]
        );
    }
}
