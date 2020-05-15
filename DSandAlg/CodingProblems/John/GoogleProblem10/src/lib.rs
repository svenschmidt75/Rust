use std::cmp;

fn find_number_of_enemies_destroyed(grid: &[u32], nrows: usize, ncols: usize) -> u32 {
    // SS: Given a grid with enemies (1) and walls (2). Find the maximum number of enemies
    // a bomb can destroy. A bomb acts horizontally and vertically only. A bomb cannot
    // penetrate a wall.

    // Solution approach: memoization via two passes. The first scans the row-wise from
    // the top to find the number of enemies reachable from that cell, the 2nd scans
    // the grid column-by-column and does the same.

    let mut memoization_grid = vec![0; ncols * nrows];

    // SS: 1st pass
    for row in 0..nrows {
        let mut start_col = 0;
        let mut enemies = 0;
        for col in 0..ncols {
            let grid_index = row * ncols + col;
            let cell = grid[grid_index];
            if cell == 2 {
                // SS: fill-in enemy count
                for k in start_col..col {
                    let memoization_grid_index = row * ncols + k;
                    memoization_grid[memoization_grid_index] = enemies;

                    println!("({}, {}) = {}", row, k, enemies);
                }
                start_col = col + 1;
            } else if cell == 1 {
                enemies += 1;
            }
        }

        // SS: fill-in enemy count
        for k in start_col..ncols {
            let memoization_grid_index = row * ncols + k;
            memoization_grid[memoization_grid_index] = enemies;

            println!("({}, {}) = {}", row, k, enemies);
        }
    }

    let mut max_enemies = 0;

    // SS: 2nd pass
    for col in 0..ncols {
        let mut start_row = 0;
        let mut enemies = 0;
        for row in 0..nrows {
            let grid_index = row * ncols + col;
            let cell = grid[grid_index];
            if cell == 2 {
                // SS: fill-in enemy count
                for k in start_row..row {
                    let memoization_grid_index = k * ncols + col;

                    let grid_index = k * ncols + col;
                    let cell = grid[grid_index];

                    let memoization_cell = &mut memoization_grid[memoization_grid_index];
                    *memoization_cell += enemies;

                    if cell == 1 {
                        *memoization_cell -= 1;
                    }

                    max_enemies = cmp::max(max_enemies, *memoization_cell);

                    println!("({}, {}) = {}", k, col, *memoization_cell);
                }
                start_row = row + 1;
            } else if cell == 1 {
                enemies += 1;
            }
        }

        // SS: fill-in enemy count
        for k in start_row..nrows {
            let memoization_grid_index = k * ncols + col;

            let grid_index = k * ncols + col;
            let cell = grid[grid_index];

            let memoization_cell = &mut memoization_grid[memoization_grid_index];
            *memoization_cell += enemies;

            if cell == 1 {
                *memoization_cell -= 1;
            }

            max_enemies = cmp::max(max_enemies, *memoization_cell);

            println!("({}, {}) = {}", k, col, *memoization_cell);
        }
    }

    max_enemies
}

#[cfg(test)]
mod tests {
    use crate::find_number_of_enemies_destroyed;

    #[test]
    fn test1() {
        // Arrange
        let grid = [0, 0, 2, 0, 0,
            0, 1, 0, 1, 1,
            0, 2, 1, 0, 0,
            0, 0, 0, 2, 0,
            0, 0, 1, 0, 0,];

        // Act
        let nenemies = find_number_of_enemies_destroyed(&grid, 5, 5);

        // Assert
        assert_eq!(nenemies, 5);
    }

    #[test]
    fn test2() {
        // Arrange
        let grid = [0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                             0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                             0, 2, 1, 0, 0, 0, 1, 1, 1, 1,
                             0, 1, 0, 1, 0, 0, 1, 0, 2, 0,
                             0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                             0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
                             0, 0, 0, 2, 0, 1, 2, 0, 0, 0,
                             0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                             0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                             2, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ];

        // Act
        let nenemies = find_number_of_enemies_destroyed(&grid, 10, 10);

        // Assert
        assert_eq!(nenemies, 6);
    }
}
