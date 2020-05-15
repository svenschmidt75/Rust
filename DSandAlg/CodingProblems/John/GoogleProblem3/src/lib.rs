use std::cmp;
use std::collections::{HashSet, VecDeque};

fn has_solution_1(obstacles: &[bool], j: usize, k: usize) -> bool {
    // SS: Given a slice with obstacles (true), find whether there is a
    // path all the way to the end.
    // You can either not jump by advancing one step, or you can jump
    // x steps, where x in [j, k].

    // SS: Solution approach is breadth-first-ish...

    // SS: total runtime: O(n * (k - j + 1))
    // O(n) memory

    // TODO SS: validate input

    let mut targets = VecDeque::new();
    targets.push_front(0);

    let mut visited = HashSet::new();
    visited.insert(0);

    // SS: O(n)
    while targets.is_empty() == false {
        let position = targets.pop_back().unwrap();

        if position == obstacles.len() - 1 {
            // SS: we reached the end
            return true;
        }

        // SS: do not jump
        if obstacles[position + 1] == false {
            if visited.contains(&(position + 1)) == false {
                targets.push_front(position + 1);
                visited.insert(position + 1);
            }
        }

        // SS: add all positions reachable from here
        // SS: O(k - j + 1)
        let max_pos = cmp::min(obstacles.len() - 1, position + k);
        for p in (position + j)..=max_pos {
            if obstacles[p] == false {
                if visited.contains(&p) == false {
                    targets.push_front(p);
                    visited.insert(p);
                }
            }
        }
    }

    false
}

fn has_solution_2(obstacles: &[bool], j: usize, k: usize) -> bool {
    // SS: Given a slice with obstacles (true), find whether there is a
    // path all the way to the end.
    // You can either not jump by advancing one step, or you can jump
    // x steps, where x in [j, k].

    // TODO SS: validate input

    let mut safety = vec![-1; obstacles.len()];
    safety[0] = 0;

    // SS: O(n)
    for position in 0..(obstacles.len() - 1) {
        if obstacles[position] == true {
            continue;
        }

        // SS: invalid position
        if safety[position] == -1 {
            continue;
        }

        safety[position] = position as i32;

        // SS: do not jump
        if obstacles[position + 1] == false {
            safety[position + 1] = position as i32;
        }

        // SS: add all positions reachable from here
        // SS: O(k - j + 1)
        let max_pos = cmp::min(obstacles.len() - 1, position + k);
        for p in (position + j)..=max_pos {
            if obstacles[p] == false {
                safety[p] = position as i32;
            }
        }
    }

    safety[obstacles.len() - 1] > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test11() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_1(&obstacles, 1, 1);

        // Assert
        assert_eq!(hash_solution, false);
    }

    #[test]
    fn test12() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_1(&obstacles, 3, 4);

        // Assert
        assert_eq!(hash_solution, true);
    }

    #[test]
    fn test13() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_1(&obstacles, 5, 5);

        // Assert
        assert_eq!(hash_solution, false);
    }

    #[test]
    fn test14() {
        // Arrange
        let obstacles = [false, true, true, true, false, true, false, false];

        // Act
        let hash_solution = has_solution_1(&obstacles, 5, 5);

        // Assert
        assert_eq!(hash_solution, false);
    }

    #[test]
    fn test21() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_2(&obstacles, 1, 1);

        // Assert
        assert_eq!(hash_solution, false);
    }

    #[test]
    fn test22() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_2(&obstacles, 3, 4);

        // Assert
        assert_eq!(hash_solution, true);
    }

    #[test]
    fn test23() {
        // Arrange
        let obstacles = [
            false, false, false, false, false, true, false, false, false, true, false, false, true,
            true, false, false,
        ];

        // Act
        let hash_solution = has_solution_2(&obstacles, 5, 5);

        // Assert
        assert_eq!(hash_solution, false);
    }

    #[test]
    fn test24() {
        // Arrange
        let obstacles = [false, true, true, true, false, true, false, false];

        // Act
        let hash_solution = has_solution_2(&obstacles, 4, 4);

        // Assert
        assert_eq!(hash_solution, false);
    }
}
