/* Grid with N cells. Marbles in the grid. If two marbles for a horizontal or vertical line,
   you may remove one of them. You continue until there are no more marbles that form a
   horizontal or vertical line. Compute the max number of marbles you can remove.
*/

mod pq;

use crate::pq::PriorityQueue;
use std::collections::HashMap;
use std::ptr::hash;

fn from_flat_index(index: usize, ncols: usize) -> (usize, usize) {
    let row = index / ncols;
    let col = index - row * ncols;
    (row, col)
}

fn solve_non_graph(grid: &[u8], nrows: usize, ncols: usize) -> u32 {
    // SS: solve without using graph

    let mut pq = PriorityQueue::new();

    // SS: pre-processing, O(nrows * ncols * (nrows + ncols))
    // so we can look up marbles by row and column
    let mut marbles_by_row = HashMap::new();
    let mut marbles_by_col = HashMap::new();

    let mut marbles = vec![];
    for i in 0..grid.len() {
        let cell = grid[i];
        if cell == 1 {
            // SS: found marble
            let (row, col) = from_flat_index(i, ncols);
            marbles.push((row, col));

            let interaction_number = marbles_by_row.entry(row).or_insert(0);
            *interaction_number += 1;

            let interaction_number = marbles_by_col.entry(col).or_insert(0);
            *interaction_number += 1;
        }
    }

    // SS: for fast lookup
    let mut mbr = HashMap::new();
    let mut mbc = HashMap::new();

    let mut marbles_hash = HashMap::new();

    // SS: push all marbles onto the min PQ
    for (row, col) in marbles {
        let mut interaction_number = 0;
        let &nr = marbles_by_row.get(&row).unwrap();
        if nr > 0 {
            interaction_number = nr - 1;
        }

        let &nc = marbles_by_col.get(&col).unwrap();
        if nc > 0 {
            interaction_number += nc - 1;
        }

        pq.insert(interaction_number as i64, (row, col));

        let hash_key = (row, col);
        marbles_hash.insert(hash_key, (row, col, interaction_number));

        let ms = mbr.entry(row).or_insert(vec![]);
        ms.push(hash_key);

        let ms = mbc.entry(col).or_insert(vec![]);
        ms.push(hash_key);
    }

    let mut max_removed = 0;

    while pq.is_empty() == false {
        let (interaction_number, (row, col)) = pq.pop();

        // SS: decrease the interaction numbers of all other marbles in the same row/col
        // and remove the current one.
        let marbles = mbr.get_mut(&row).unwrap();
        for key in marbles {
            let (other_row, other_col, other_interaction_number) =
                marbles_hash.get_mut(&key).unwrap();
            if *other_col == col && *other_row == row && *other_interaction_number > 0 {
                *other_interaction_number = -1;
                max_removed += 1;
            } else {
                *other_interaction_number -= 1;
                if *other_interaction_number > 0 {
                    pq.insert(*other_interaction_number as i64, (*other_row, *other_col));
                }
            }
        }

        let marbles = mbc.get_mut(&col).unwrap();
        for key in marbles {
            let (other_row, other_col, other_interaction_number) =
                marbles_hash.get_mut(&key).unwrap();
            if *other_col == col && *other_row == row {
            } else {
                *other_interaction_number -= 1;
                if *other_interaction_number > 0 {
                    pq.insert(*other_interaction_number as i64, (*other_row, *other_col));
                }
            }
        }
    }

    max_removed
}

#[cfg(test)]
mod tests {
    use crate::solve_non_graph;

    #[test]
    fn it_works() {
        // Arrange
        let grid = [
            1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
        ];

        // Act
        let max_remove = solve_non_graph(&grid, 5, 5);

        // Assert
        assert_eq!(max_remove, 6);
    }
}
