// https://www.reddit.com/r/CodingProblems/comments/f3tyb1/day_2_20200214_problem_of_the_day_asked_by/

use std::collections::{HashSet, VecDeque};

fn count_islands(grid: &[u8], nrows: u8, ncols: u8) -> u8 {
    // SS: use BF approach (think flood-fill)
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let mut nislands = 0;

    for col in 0..ncols {
        for row in 0..nrows {
            let index = flatten_index(row, col, ncols);
            if visited.contains(&index) {
                continue;
            }
            visited.insert(index);

            let value = grid[index];

            if value == 1 {
                // SS: found part of island
                nislands += 1;

                // SS: mark all neighboring 1's
                let mut neighbors = get_neighbors(row, col, nrows, ncols);
                neighbors.into_iter().for_each(|(x, y)| {
                    queue.push_back((x, y));
                });

                while queue.is_empty() == false {
                    let (x, y) = queue.pop_front().unwrap();
                    let idx = flatten_index(x, y, ncols);
                    if visited.contains(&idx) {
                        continue;
                    }
                    visited.insert(idx);

                    let value = grid[idx];
                    if value == 1 {
                        let mut neighbors = get_neighbors(x, y, nrows, ncols);
                        neighbors.into_iter().for_each(|(x, y)| {
                            queue.push_back((x, y));
                        });
                    }
                }
            }
        }
    }
    nislands
}

fn get_neighbors(row: u8, col: u8, nrows: u8, ncols: u8) -> Vec<(u8, u8)> {
    let mut neighbors = vec![];

    if col as i16 - 1 >= 0 {
        neighbors.push((row, col - 1));
    }

    if col + 1 < ncols {
        neighbors.push((row, col + 1));
    }

    if row as i16 - 1 >= 0 {
        neighbors.push((row - 1, col));
    }

    if row + 1 < nrows {
        neighbors.push((row + 1, col));
    }

    neighbors
}

fn flatten_index(row: u8, col: u8, ncols: u8) -> usize {
    (row * ncols + col) as usize
}

#[cfg(test)]
mod tests {
    use crate::count_islands;

    #[test]
    fn test1() {
        // Arrange

        /* 1 0 0 0 1
         * 1 1 0 0 0
         * 1 0 1 1 0
         * 0 0 0 0 0
         */
        let grid = [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0];
        let ncols = 5;
        let nrows = 4;

        // Act
        let island_count = count_islands(&grid, nrows, ncols);

        // Assert
        assert_eq!(island_count, 3);
    }
}
