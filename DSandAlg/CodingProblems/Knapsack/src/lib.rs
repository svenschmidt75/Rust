use std::cmp;

fn grid_index(row: usize, col: usize, row_size: usize) -> usize {
    row * row_size + col
}

fn knapsack_1(input: &[(usize, i64)], size: usize) -> i64 {
    let mut grid = vec![0; input.len() * size];

    for item in 0..input.len() {
        // SS: check whether to steal the current item or not
        for knapsack_size in 1..=size {
            let mut new_max = 0;

            // SS: does this item fit in a sub-knapsack of size 'size'?
            if input[item].0 <= knapsack_size {
                // SS: yes, it does
                new_max = input[item].1;
            }

            // SS: take the optimal case from previous sub problem and SAME sub knapsack size
            if item > 0 && knapsack_size > input[item].0 {
                let prev_col = size - input[item].0 - 1;
                let prev_optimum = grid[grid_index(item - 1, prev_col, size)];
                new_max += prev_optimum;
            }

            let prev_size_optimum = if item > 0 {
                grid[grid_index(item - 1, knapsack_size - 1, size)]
            } else {
                0
            };

            let new_optimum = cmp::max(new_max, prev_size_optimum);
            grid[grid_index(item, knapsack_size - 1, size)] = new_optimum;
        }
    }

    let global_optimum = grid[grid_index(input.len() - 1, size - 1, size)];
    global_optimum
}

#[cfg(test)]
mod tests {
    use crate::knapsack_1;

    #[test]
    fn test1() {
        // Arrange
        let input = [(1, 1500), (4, 3000), (3, 2000)];

        // Act
        let result = knapsack_1(&input, 4);

        // Assert
        assert_eq!(result, 3500);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [(1, 1500), (4, 3000), (3, 2000), (1, 2000)];

        // Act
        let result = knapsack_1(&input, 4);

        // Assert
        assert_eq!(result, 4000);
    }
}
