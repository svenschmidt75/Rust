use std::collections::{HashSet, VecDeque};

fn from_flat_index(position: i64, nrows: usize, ncols: usize) -> (i64, i64) {
    let row = position / ncols as i64;
    let col = position - row * nrows as i64;
    (row as i64, col as i64)
}

fn to_flat_index(row: i64, col: i64, nrows: usize) -> i64 {
    row * nrows as i64 + col
}

fn get_manhattan_distance(from: i64, to: i64, nrows: usize, ncols: usize) -> u64 {
    let (from_row, from_col) = from_flat_index(from, nrows, ncols);
    let (to_row, to_col) = from_flat_index(to, nrows, ncols);
    let dist = (from_row - to_row).abs() + (from_col - to_col).abs();
    dist as u64
}

fn get_neighbors(position: i64, nrows: usize, ncols: usize) -> Vec<i64> {
    let (row, col) = from_flat_index(position, nrows, ncols);

    let mut neighbors = vec![];

    if row - 1 >= 0 {
        let index = to_flat_index(row - 1, col, nrows);
        neighbors.push(index);
    }

    if row + 1 < nrows as i64 {
        let index = to_flat_index(row + 1, col, nrows);
        neighbors.push(index);
    }

    if col - 1 >= 0 {
        let index = to_flat_index(row, col - 1, nrows);
        neighbors.push(index);
    }

    if col + 1 < ncols as i64 {
        let index = to_flat_index(row, col + 1, nrows);
        neighbors.push(index);
    }

    neighbors
}

fn find_min_manhattan_distance(grid: &[u8], nrows: usize, ncols: usize) -> u64 {
    // SS: extract all x's, then do multi-point breadth-first search
    // at runtime O(R * C), R=rows, C=columns

    let mut visited = HashSet::new();

    // SS: insert all x's into the queue
    let mut queue = VecDeque::new();
    for position in 0..grid.len() {
        let item = grid[position];
        if item == 1 {
            queue.push_back((position as i64, position as i64));
            visited.insert(position as i64);
        }
    }

    while queue.is_empty() == false {
        let (position, start_position) = queue.pop_front().unwrap();
        let item = grid[position as usize];
        if item == 2 {
            // SS: the first y found is the closest one to any x
            let manhattan_distance = get_manhattan_distance(start_position, position, nrows, ncols);
            return manhattan_distance;
        }

        let neighbors = get_neighbors(position, nrows, ncols);
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(neighbor);
            queue.push_back((neighbor, start_position));
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_test1() {
        // Arrange
        let grid = [
            1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];

        // Act
        let min_distance = find_min_manhattan_distance(&grid, 5, 5);

        // Assert
        assert_eq!(min_distance, 3);
    }
}
