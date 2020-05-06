// Data Structures & Algorithms ! - https://www.udemy.com/course/draft/1330262/learn/lecture/14237684#overview
// S12.4 - L13 --  Ways to Reach last cell

use std::cmp;

fn min_cost(
    grid: &[u32],
    nrows: usize,
    ncols: usize,
    remaining_cost: u32,
    current_row: usize,
    current_col: usize,
) -> u32 {
    // SS: base case
    if current_row == nrows - 1 && current_col == ncols - 1 {
        let cost = grid[grid.len() - 1];
        if cost == remaining_cost {
            // SS: we found a path
            1
        } else {
            0
        }
    } else if current_row == nrows || current_col == ncols {
        0
    } else {
        let idx = current_row * ncols + current_col;
        let cost = grid[idx];
        if remaining_cost >= cost {
            let c1 = min_cost(
                grid,
                nrows,
                ncols,
                remaining_cost - cost,
                current_row + 1,
                current_col,
            );
            let c2 = min_cost(
                grid,
                nrows,
                ncols,
                remaining_cost - cost,
                current_row,
                current_col + 1,
            );
            c1 + c2
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_down() {
        // Arrange
        let grid = [4, 7, 1, 6, 5, 7, 3, 9, 3, 2, 1, 2, 7, 1, 6, 3];

        // Arrange
        let min_cost = min_cost(&grid, 4, 4, 25, 0, 0);

        // Assert
        assert_eq!(min_cost, 2);
    }
}
