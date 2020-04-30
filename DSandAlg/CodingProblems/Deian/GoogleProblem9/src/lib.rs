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

fn follow_up_1_visit(
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

        let distance = follow_up_1_visit(neighbor, cum_dst + dst, v, distances);
        min_cum_dst = cmp::min(min_cum_dst, distance);

        i += 1;
    }

    min_cum_dst
}

fn follow_up_1(start: (i64, i64), x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> u64 {
    /* Pick up 3 persons x, y and z from any start point s.t. the distance
     * travelled is minimal...
     * Implemented as greedy algorithm: at each vertex, follow the smallest edge...
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

    follow_up_1_visit(0, 0, visited, &distances)
}

struct Node {
    level: u8,
    split_value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    point: Option<(i64, i64)>,
}

impl Node {
    fn new(level: u8, split_value: i64) -> Node {
        Node {
            level,
            split_value,
            left: None,
            right: None,
            point: None,
        }
    }
}

fn create_node(
    ys: &[(i64, i64)],
    level: u8,
    (xmin_idx, xmax_idx): (i64, i64),
    (ymin_idx, ymax_idx): (i64, i64),
) -> Option<Box<Node>> {
    // SS: remove code duplication...

    // SS: We construct the kd-tree such that a each point has its own leaf node.

    if ys.is_empty() {
        None
    } else if level % 2 == 0 {
        // SS: on even levels, we split x (i.e. by column) into [xmin, mid), [mid, xmax)
        let mid_idx = (ys.len() / 2) as i64;
        if mid_idx == 0 {
            // SS: leaf node
            let mut node = Node::new(level, xmin_idx);
            node.point = Some(ys[0]);
            Some(Box::new(node))
        } else {
            // SS: partition points into two halves according to split value
            let split_value = ys[mid_idx as usize - 1].1;
            let mut left_nodes = vec![];
            let mut right_nodes = vec![];
            for i in 0..ys.len() {
                let y = ys[i as usize];
                if y.1 <= split_value {
                    left_nodes.push(y);
                } else {
                    right_nodes.push(y);
                }
            }

            let left = create_node(
                &left_nodes,
                level + 1,
                (xmin_idx, xmin_idx + mid_idx),
                (ymin_idx, ymax_idx),
            );
            let right = create_node(
                &right_nodes,
                level + 1,
                (xmin_idx + mid_idx, xmax_idx),
                (ymin_idx, ymax_idx),
            );
            let mut node = Node::new(level, split_value);
            node.left = left;
            node.right = right;
            Some(Box::new(node))
        }
    } else {
        // SS: on odd levels, we split y (i.e. by row) into [ymin, mid), [mid, ymax)
        let mid_idx = (ys.len() / 2) as i64;
        if mid_idx == 0 {
            // SS: leaf node
            let mut node = Node::new(level, ymin_idx);
            node.point = Some(ys[0]);
            Some(Box::new(node))
        } else {
            // SS: partition points into two halves according to split value
            let split_value = ys[mid_idx as usize - 1].0;
            let mut up_nodes = vec![];
            let mut down_nodes = vec![];
            for i in 0..ys.len() {
                let y = ys[i as usize];
                if y.0 <= split_value {
                    up_nodes.push(y);
                } else {
                    down_nodes.push(y);
                }
            }

            let left = create_node(
                &up_nodes,
                level + 1,
                (xmin_idx, xmax_idx),
                (ymin_idx, ymin_idx + mid_idx),
            );
            let right = create_node(
                &down_nodes,
                level + 1,
                (xmin_idx, xmax_idx),
                (ymin_idx + mid_idx, ymax_idx),
            );
            let mut node = Node::new(level, split_value);
            node.left = left;
            node.right = right;
            Some(Box::new(node))
        }
    }
}

fn nearest_neighbor(
    node: &Node,
    (row, col): (i64, i64),
    level: u8,
    dst: &mut u64,
    best: &mut (i64, i64),
) {
    if node.point.is_some() {
        // SS: leaf node
        let manhattan_dst = get_manhattan_distance_from_pair(node.point.unwrap(), (row, col));
        if manhattan_dst < *dst {
            *dst = manhattan_dst;
            *best = node.point.unwrap();
        }
    } else if level % 2 == 0 {
        // SS: split x axis
        if col - *dst as i64 <= node.split_value as i64 {
            // SS: search left subtree
            if node.left.is_some() {
                nearest_neighbor(
                    node.left.as_ref().unwrap(),
                    (row, col),
                    level + 1,
                    dst,
                    best,
                );
            }
        }

        if col + *dst as i64 > node.split_value as i64 {
            // SS: search right subtree
            if node.right.is_some() {
                nearest_neighbor(
                    node.right.as_ref().unwrap(),
                    (row, col),
                    level + 1,
                    dst,
                    best,
                );
            }
        }
    } else {
        // SS: split y axis
        if row - *dst as i64 <= node.split_value as i64 {
            // SS: search left subtree
            if node.left.is_some() {
                nearest_neighbor(
                    node.left.as_ref().unwrap(),
                    (row, col),
                    level + 1,
                    dst,
                    best,
                );
            }
        }

        if row + *dst as i64 > node.split_value as i64 {
            // SS: search right subtree
            if node.right.is_some() {
                nearest_neighbor(
                    node.right.as_ref().unwrap(),
                    (row, col),
                    level + 1,
                    dst,
                    best,
                );
            }
        }
    }
}

fn create_kdtree(ys: &[(i64, i64)]) -> Option<Box<Node>> {
    create_node(ys, 0, (0, ys.len() as i64), (0, ys.len() as i64))
}

fn follow_up_2(xs: &[(i64, i64)], ys: &[(i64, i64)]) -> u64 {
    /* Given a set of xs and ys, find the smallest Manhattan distance
     * between any one x and y.
     * Create kd-tree for the ys and implement closest neighbor search
     * for each x in xs.
     * Total runtime:
     *   construction of kd-tree: O(Y log Y)
     *   nearest neighbor search: O(X log Y)
     * => O( (X + Y) log Y)
     */

    let root = create_kdtree(ys).unwrap();

    let mut global_best = 2 * ys.len() as u64;
    let mut best = (0, 0);
    for &x in xs {
        let mut dst = global_best;
        nearest_neighbor(&root, x, 0, &mut dst, &mut best);
        global_best = cmp::min(global_best, dst);
    }

    global_best
}

fn original_problem_memoization(grid: &[u8], nrows: usize, ncols: usize) -> u64 {
    // SS: use a memoization approach
    // We do two passes: left-right, top-bottom and right-left, bottom-top.
    // At each cell, we store two values: distance from closest x and distance
    // from closest y.
    // The solution is given by the smallest x or y at an x or y cell.
    // Runtime is O(R * C), R=rows, C=columns

    let mut memoization_grid = vec![0; nrows * ncols * 2];

    // SS: 1st pass
    // The update rule for the first pass is:
    // cell[j, i] = min(cell[j, i - 1], cell[j - 1, i])

    let mut closest_y_dist = i32::MAX;

    for j in 0..nrows {
        let mut closest_x_dist = i32::MAX;

        for i in 0..ncols {
            let grid_index = j * ncols + i;
            let memoization_grid_index = (j * 2 * ncols) + 2 * i;

            let cell = grid[grid_index];
            if cell == 1 {
                // SS: X
                closest_x_dist = 0;
            } else {
                let mut value = closest_x_dist;

                if j == 0 && i > 0 {
                    // SS: 1st row

                    // cell[row, col - 1]
                    let memoization_grid_index = 2 * (i - 1);
                    let cell = memoization_grid[memoization_grid_index];
                    closest_x_dist = cell;
                    if cell < i32::MAX {
                        closest_x_dist += 1;
                    }
                } else if j > 0 && i == 0 {
                    // SS: 1st column

                    // cell[row - 1, col]
                    let memoization_grid_index = (j - 1) * 2 * ncols;
                    let cell = memoization_grid[memoization_grid_index];
                    closest_x_dist = cell;
                    if cell < i32::MAX {
                        closest_x_dist += 1;
                    }
                }
                else if i > 0 && j > 0
                {
                    // cell[row - 1, col]
                    let memoization_grid_index = ((j - 1) * 2 * ncols) + 2 * i;
                    let cell1 = memoization_grid[memoization_grid_index];

                    // cell[row, col - 1]
                    let memoization_grid_index = (j * 2 * ncols) + 2 * (i - 1);
                    let cell2 = memoization_grid[memoization_grid_index];

                    if cell1 == i32::MAX && cell2 == i32::MAX {
                        value = i32::MAX;
                    } else {
                        value = cmp::min(cell1, cell2);
                        value = cmp::min(closest_x_dist, value) + 1;
                    }
                }

                closest_x_dist = value;
            }

            memoization_grid[memoization_grid_index] = closest_x_dist;

            println!("({}, {}) = {}", j, i, closest_x_dist);
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

    #[test]
    fn problem1_test2() {
        // Arrange
        let grid = [
            0, 0, 0, 0, 0, 1,
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0,
            0, 0, 0, 0, 0, 2,
            0, 0, 1, 0, 0, 0,
        ];

        // Act
        let min_distance = original_problem_memoization(&grid, 5, 6);

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

    #[test]
    fn problem3_test1() {
        // Arrange
        let xs = [(0, 0), (1, 2)];
        let ys = [(1, 1), (3, 1), (4, 2), (3, 4)];

        // Act
        let min_distance = follow_up_2(&xs, &ys);

        // Assert
        assert_eq!(min_distance, 1);
    }
}
