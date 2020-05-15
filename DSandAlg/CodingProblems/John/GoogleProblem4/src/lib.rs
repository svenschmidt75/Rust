/* Given a 2D matrix with Booleans. “True” means that there is land at that tile. “False” means that there is water.
 * Assign a height to every tile so that:
 *   Water is always zero.
 *   Two adjacent tiles don’t differ by more than 1.
 * Report the height of the highest peak you can possibly build.

Example:

T T T F
F T T F
T T T F

1 2 1 0
0 1 1 0
1 2 1 0

*/

use std::collections::{VecDeque, HashSet};
use std::cmp;


fn from_flat_index(index: i32, nrows: i32, ncols: i32) -> (i32, i32){
    let row = index / ncols;
    let col = index - row * ncols;
    if row >= nrows || col >= ncols {
        (-1, -1)
    } else {
        (row, col)
    }
}

fn to_flat_index(row: i32, col: i32, nrows: i32, ncols: i32) -> i32 {
    if row < 0 || col < 0 || row >= nrows || col >= ncols {
        -1
    } else {
        row * ncols + col
    }
}

fn get_neighbors(index: i32, nrows: i32, ncols: i32) -> Vec<i32> {
    let mut neighbors = vec![];
    let (row, col) = from_flat_index(index, nrows, ncols);

    let mut index_neighbor = to_flat_index(row - 1, col, nrows, ncols);
    if index_neighbor > -1 {
        neighbors.push(index_neighbor);
    }

    index_neighbor = to_flat_index(row + 1, col, nrows, ncols);
    if index_neighbor > -1 {
        neighbors.push(index_neighbor);
    }

    index_neighbor = to_flat_index(row, col - 1, nrows, ncols);
    if index_neighbor > -1 {
        neighbors.push(index_neighbor);
    }

    index_neighbor = to_flat_index(row, col + 1, nrows, ncols);
    if index_neighbor > -1 {
        neighbors.push(index_neighbor);
    }

    neighbors
}

fn find_max_height(input: &[bool], nrows: i32, ncols: i32) -> usize {
    // SS: use breadth-first approach, depth-first wouldn't work
    // due to the max 1 difference requirement...
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let mut height_matrix = vec![0; input.len()];
    for i in 0..input.len() {
        match input[i] {
            true => {
                // SS: land
                height_matrix[i] = -1;
            },
            false => {
                // SS: water
                height_matrix[i] = 0;
                queue.push_front((i as i32, 0));
                visited.insert(i as i32);
            }
        }
    }

    let mut max_height = 0;

    while queue.is_empty() == false {
        let (index, height) = queue.pop_back().unwrap();
        max_height = cmp::max(max_height, height);

        let neighbors = get_neighbors(index, nrows, ncols);
        for neighbor in neighbors {
            if visited.contains(&neighbor) == false {
                queue.push_front((neighbor, height + 1));
                visited.insert(neighbor);
            }
        }
    }

    max_height
}

#[cfg(test)]
mod tests {
    use crate::find_max_height;

    #[test]
    fn test1() {
        // Arrange
        let input = [true, true, true, false, true, true, true, false, true, true, true, false];
        let nrows = 3;
        let ncols = 4;

        // Act
        let max_height = find_max_height(&input, nrows, ncols);

        // Assert
        assert_eq!(max_height, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let input = [true, true, true, false, false, true, true, false, true, true, true, false];
        let nrows = 3;
        let ncols = 4;

        // Act
        let max_height = find_max_height(&input, nrows, ncols);

        // Assert
        assert_eq!(max_height, 2);
    }
}
