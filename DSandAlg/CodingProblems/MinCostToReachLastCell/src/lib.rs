// Data Structures & Algorithms ! - https://www.udemy.com/course/draft/1330262/learn/lecture/13950508#overview
// S12.4 - L12 --  Min Cost to Reach End of Array

use std::cmp;

fn min_cost(
    grid: &[u32],
    nrows: usize,
    ncols: usize,
    current_row: usize,
    current_col: usize,
) -> u64 {
    // SS: base case
    if current_row == nrows - 1 && current_col == ncols - 1 {
        let cost = grid[grid.len() - 1] as u64;
        cost
    } else if current_row == nrows || current_col == ncols {
        // SS: also invalid base case, penalize
        1_000_000
    } else {
        let idx = current_row * ncols + current_col;
        let cost = grid[idx] as u64;

        let c1 = min_cost(grid, nrows, ncols, current_row + 1, current_col);
        let c2 = min_cost(grid, nrows, ncols, current_row, current_col + 1);

        cost + cmp::min(c1, c2)
    }
}

fn min_cost_bottom_up(grid: &[u32], nrows: usize, ncols: usize) -> u32 {
    let mut memoization_grid = vec![0; grid.len()];
    memoization_grid[grid.len() - 1] = grid[grid.len() - 1];

    for i in 0..nrows {
        let row = nrows - 1 - i;

        for j in 0..ncols {
            let col = ncols - 1 - j;

            let index = row * ncols + col;
            let cost = grid[index];

            let mut cell_cost1 = u32::MAX;
            let mut cell_cost2 = u32::MAX;

            if row < nrows - 1 {
                let index = (row + 1) * ncols + col;
                let cost = memoization_grid[index];
                cell_cost1 = cost;
            }

            if col < ncols - 1 {
                let index = row * ncols + (col + 1);
                let cost = memoization_grid[index];
                cell_cost2 = cost;
            }

            if row == nrows - 1 && col == ncols - 1 {
                cell_cost1 = 0;
            }

            memoization_grid[index] = cost + cmp::min(cell_cost1, cell_cost2);
        }
    }

    memoization_grid[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_down() {
        // Arrange
        let grid = [
            4, 7, 8, 6, 4, 6, 7, 3, 9, 2, 3, 8, 1, 2, 4, 7, 1, 7, 3, 7, 2, 9, 8, 9, 3,
        ];

        // Arrange
        let min_cost = min_cost(&grid, 5, 5, 0, 0);

        // Assert
        assert_eq!(min_cost, 36);
    }

    #[test]
    fn test_bottom_up() {
        // Arrange
        let grid = [
            4, 7, 8, 6, 4, 6, 7, 3, 9, 2, 3, 8, 1, 2, 4, 7, 1, 7, 3, 7, 2, 9, 8, 9, 3,
        ];

        // Arrange
        let min_cost = min_cost_bottom_up(&grid, 5, 5);

        // Assert
        assert_eq!(min_cost, 36);
    }
}
