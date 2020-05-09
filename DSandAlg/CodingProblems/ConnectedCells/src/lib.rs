// https://www.youtube.com/watch?v=R4Nh-EgWjyQ&list=PLI1t_8YX-ApvMthLj56t1Rf-Buio5Y8KL&index=5

use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

fn from_flat_index(index: usize, ncols: usize) -> (usize, usize) {
    let row = index / ncols;
    let col = index - row * ncols;
    (row, col)
}

fn to_flat_index(row: usize, col: usize, ncols: usize) -> usize {
    row * ncols + col
}

fn get_neighbors(index: usize, grid: &[u8], nrows: usize, ncols: usize) -> Vec<usize> {
    let mut neighbors = vec![];

    let (row, col) = from_flat_index(index, ncols);

    if row > 0 {
        if col > 0 {
            let idx = to_flat_index(row - 1, col - 1, ncols);
            neighbors.push(idx);
        }

        let idx = to_flat_index(row - 1, col, ncols);
        neighbors.push(idx);

        if col < ncols - 1 {
            let idx = to_flat_index(row - 1, col + 1, ncols);
            neighbors.push(idx);
        }
    }

    if col > 0 {
        let idx = to_flat_index(row, col - 1, ncols);
        neighbors.push(idx);
    }

    if col < ncols - 1 {
        let idx = to_flat_index(row, col + 1, ncols);
        neighbors.push(idx);
    }

    if row < nrows - 1 {
        if col > 0 {
            let idx = to_flat_index(row + 1, col - 1, ncols);
            neighbors.push(idx);
        }

        let idx = to_flat_index(row + 1, col, ncols);
        neighbors.push(idx);

        if col < ncols - 1 {
            let idx = to_flat_index(row + 1, col + 1, ncols);
            neighbors.push(idx);
        }
    }

    neighbors
}

fn bfs(grid: &[u8], nrows: usize, ncols: usize) -> u8 {
    let mut max_cluster_size = 0;

    let mut queue = VecDeque::new();

    let mut to_visit = HashSet::new();

    // SS: put all 1s into the queue
    for index in 0..grid.len() {
        let item = grid[index];
        if item == 1 {
            queue.push_back(index);
            to_visit.insert(index);
        }
    }

    while queue.is_empty() == false {
        let index = queue.pop_front().unwrap();
        if to_visit.contains(&index) == false {
            // SS: we have already processed this 1, i.e. it is part of a cluster we already
            // processed
            continue;
        }
        to_visit.remove(&index);

        // SS: process local cluster
        let mut local_cluster_size = 1;

        let mut local_queue = VecDeque::new();
        local_queue.push_back(index);

        let mut local_visit = HashSet::new();
        local_visit.insert(index);

        while local_queue.is_empty() == false {
            let local_index = local_queue.pop_front().unwrap();
            let neighbors = get_neighbors(local_index, grid, nrows, ncols);
            for neighbor in neighbors {
                let item = grid[neighbor];
                if item == 1 {
                    if local_visit.contains(&neighbor) {
                        continue;
                    }
                    local_visit.insert(neighbor);
                    local_queue.push_back(neighbor);
                    local_cluster_size += 1;

                    // SS: do not visit in outer loop
                    to_visit.remove(&neighbor);
                }
            }
        }
        max_cluster_size = cmp::max(max_cluster_size, local_cluster_size);
    }

    max_cluster_size
}

fn dfs(grid: &[u8], nrows: usize, ncols: usize) -> usize {
    let mut queue = VecDeque::new();

    let mut to_visit = HashSet::new();

    // SS: put all 1s into the queue
    for index in 0..grid.len() {
        let item = grid[index];
        if item == 1 {
            queue.push_back(index);
            to_visit.insert(index);
        }
    }

    let mut max_cluster_size = 0;

    while queue.is_empty() == false {
        let index = queue.pop_front().unwrap();
        if to_visit.contains(&index) == false {
            // SS: we have already processed this 1, i.e. it is part of a cluster we already
            // processed
            continue;
        }
        to_visit.remove(&index);

        let mut visited = HashSet::new();
        visited.insert(index);

        let local_cluster_size = dfs_recursive(grid, nrows, ncols, &mut visited, index, 1);
        max_cluster_size = cmp::max(max_cluster_size, local_cluster_size);

        for idx in visited {
            to_visit.remove(&idx);
        }
    }

    max_cluster_size
}

fn dfs_recursive(
    grid: &[u8],
    nrows: usize,
    ncols: usize,
    visited: &mut HashSet<usize>,
    index: usize,
    depth: usize,
) -> usize {
    let mut max_cluster_size = depth;

    let neighbors = get_neighbors(index, grid, nrows, ncols);
    for neighbor in neighbors {
        let item = grid[neighbor];
        if item == 1 {
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(neighbor);

            let cluster_size =
                dfs_recursive(grid, nrows, ncols, visited, neighbor, max_cluster_size + 1);
            max_cluster_size = cmp::max(max_cluster_size, cluster_size);
        }
    }

    max_cluster_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        // Arrange
        let grid = [
            0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ];

        // Act
        let max_cluster_size = bfs(&grid, 6, 7);

        // Assert
        assert_eq!(max_cluster_size, 7);
    }

    #[test]
    fn test_dfs() {
        // Arrange
        let grid = [
            0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ];

        // Act
        let max_cluster_size = dfs(&grid, 6, 7);

        // Assert
        assert_eq!(max_cluster_size, 7);
    }
}
