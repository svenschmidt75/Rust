use std::cmp;

fn grid_index(row: usize, col: usize, row_size: usize) -> usize {
    row * row_size + col
}

fn knapsack_1(input: &[(usize, i64)], size: usize) -> i64 {
    // SS: We don't actually need to generate and keep the entire grid around since
    // we only ever look at the previous row, which is the optimum up until that
    // item. We could use two "pointers", that we change accordingly when we move
    // from the item to the next, such that the one points to the previous row,
    // the other to the current.
    let mut grid = vec![0; input.len() * size];

    // SS: each row corresponds to a new item that is available for stealing
    for item in 0..input.len() {

        // SS: solve the problem for all knapsacks until the specified size
        // These are the subproblems we solve
        for knapsack_size in 1..=size {
            let mut new_max = 0;

            // SS: does this item fit in a sub-knapsack of size 'size'?
            if input[item].0 <= knapsack_size {
                // SS: yes, it does
                new_max = input[item].1;
            }

            // SS: take the previous optimal case for the remaining knapsack size
            // after subtracting the current item from the knacksack size
            if item > 0 && knapsack_size > input[item].0 {
                let prev_col = size - input[item].0 - 1;
                let prev_optimum = grid[grid_index(item - 1, prev_col, size)];
                new_max += prev_optimum;
            }

            // SS: check whether the previous optimum for the same knapsack size is better
            // than stealing the current item
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
