use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

fn from_flat_index(position: i64, nrows: usize, ncols: usize) -> (i64, i64) {
    let row = position / ncols as i64;
    let col = position - row * nrows as i64;
    (row as i64, col as i64)
}

fn to_flat_index(row: i64, col: i64, nrows: usize) -> i64 {
    row * nrows as i64 + col
}

fn get_manhattan_distance_from_pair(
    (from_row, from_col): (i64, i64),
    (to_row, to_col): (i64, i64),
) -> u64 {
    let dist = (from_row - to_row).abs() + (from_col - to_col).abs();
    dist as u64
}

fn get_manhattan_distance(from: i64, to: i64, nrows: usize, ncols: usize) -> u64 {
    let from_pair = from_flat_index(from, nrows, ncols);
    let to_pair = from_flat_index(to, nrows, ncols);
    get_manhattan_distance_from_pair(from_pair, to_pair)
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

    // SS: insert all x's into the queue, O(R * C)
    let mut queue = VecDeque::new();
    for position in 0..grid.len() {
        let item = grid[position];
        if item == 1 {
            queue.push_back((position as i64, position as i64));
            visited.insert(position as i64);
        }
    }

    // SS: do breadth-first at O(R * C)
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

fn visit(
    vertex: i64,
    cum_dst: u64,
    visited: HashSet<i64>,
    distances: &HashMap<i64, [u64; 4]>,
) -> u64 {
    let mut neighbors = vec![];

    // SS: check all other vertices
    for neighbor in 0..4 {
        if visited.contains(&neighbor) {
            continue;
        }

        // SS: get distance
        let distance = distances.get(&vertex).unwrap()[neighbor as usize];
        neighbors.push((neighbor, distance));
    }

    if neighbors.is_empty() {
        // SS: reached end-of-path
        return cum_dst;
    }

    // SS: sort by distance, can be done in O(1) since array size is fixed
    neighbors.sort_by_key(|&(vertex, dist)| dist);

    let mut min_cum_dst = std::u64::MAX;

    let min_distance = neighbors[0].1;
    let mut i = 0;
    while i < neighbors.len() {
        let (neighbor, dst) = neighbors[i];
        if dst > min_distance {
            break;
        }

        // SS: check neighbor
        let mut v = visited.clone();
        v.insert(neighbor);

        let distance = visit(neighbor, cum_dst + dst, v, distances);
        min_cum_dst = cmp::min(min_cum_dst, distance);

        i += 1;
    }

    min_cum_dst
}

fn follow_up_1(start: (i64, i64), x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> u64 {
    /* Pick up 3 persons x, y and z from any start point s.t. the distance
     * travelled is minimal...
     */

    // SS: pre-compute distances (i.e. effectively the edges of a graph)
    let mut distances = HashMap::new();

    distances.insert(
        0,
        [
            0,
            get_manhattan_distance_from_pair(start, x),
            get_manhattan_distance_from_pair(start, y),
            get_manhattan_distance_from_pair(start, z),
        ],
    );

    distances.insert(
        1,
        [
            get_manhattan_distance_from_pair(start, x),
            0,
            get_manhattan_distance_from_pair(x, y),
            get_manhattan_distance_from_pair(x, z),
        ],
    );

    distances.insert(
        2,
        [
            get_manhattan_distance_from_pair(start, y),
            get_manhattan_distance_from_pair(x, y),
            0,
            get_manhattan_distance_from_pair(y, z),
        ],
    );

    distances.insert(
        3,
        [
            get_manhattan_distance_from_pair(start, z),
            get_manhattan_distance_from_pair(x, z),
            get_manhattan_distance_from_pair(y, z),
            0,
        ],
    );

    // SS: convention:
    // 0: start
    // 1: x
    // 2: y
    // 3: z
    let mut visited = HashSet::new();
    visited.insert(0);

    visit(0, 0, visited, &distances)
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

    #[test]
    fn problem2_test1() {
        // Arrange

        // Act
        let min_distance = follow_up_1((0, 0), (3, 1), (1, 3), (5, 2));

        // Assert
        assert_eq!(min_distance, 11);
    }

    #[test]
    fn problem2_test2() {
        // Arrange

        // Act
        let min_distance = follow_up_1((0, 0), (3, 1), (1, 3), (2, 2));

        // Assert
        assert_eq!(min_distance, 8);
    }
}
