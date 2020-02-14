// https://www.reddit.com/r/AlgoExpert/comments/ew89ml/day_7_20200130_problem_of_the_day_asked_by_google/

use std::cmp;
use std::collections::{HashMap, VecDeque};

fn is_on_same_diagonal(bishop1: (i32, i32), bishop2: (i32, i32)) -> bool {
    let delta = (bishop2.0 - bishop1.0).abs();
    (bishop2.1 - bishop1.1).abs() == delta
}

fn find_attacks_1(grid_size: usize, bishops: &[(i32, i32)]) -> u8 {
    let mut attacks = 0;

    // O(N^2) solution
    for i in 0..bishops.len() {
        let bishop = bishops[i];

        for j in (i + 1)..bishops.len() {
            let other_bishop = bishops[j];

            if is_on_same_diagonal(bishop, other_bishop) {
                attacks += 1;
            }
        }
    }

    attacks
}

fn find_diagonal(bishop: (i32, i32)) -> Vec<i32> {
    // SS: find the y value of the point on the both diagonals
    // with x=0
    let mut vec = vec![];
    let diagonal1 = bishop.1 - bishop.0;
    if diagonal1 >= 0 {
        vec.push(diagonal1);
    }
    let diagonal2 = bishop.1 + bishop.0;
    if diagonal1 != diagonal2 {
        vec.push(diagonal2);
    }
    vec
}

fn find_attacks_2(grid_size: usize, bishops: &[(i32, i32)]) -> usize {
    // O(N)  solution
    let mut hash: HashMap<i32, usize> = HashMap::new();

    // SS: for each bishop, find its at most two diagonals, and
    // increase a value in the hash map for both of them

    // O(N) loop
    for bishop in bishops {
        let diagonals = find_diagonal(*bishop);
        diagonals.iter().for_each(|d| {
            if let Some(x) = hash.get_mut(d) {
                *x += 1;
            } else {
                hash.insert(*d, 1);
            }
        });
    }

    // SS: count the number of bishops in each diagonal - 1
    // O(N) loop
    let attacks = hash.iter().fold(0, |acc, (x, y)| acc + y - 1);
    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Arrange
        let bishops = [(0, 0), (1, 2), (2, 2), (4, 0)];

        // Act
        let attacks = find_attacks_1(5, &bishops);

        // Assert
        assert_eq!(attacks, 2);
    }

    #[test]
    fn test2() {
        // Arrange
        let bishops = [(0, 0), (1, 2), (2, 2), (4, 0)];

        // Act
        let attacks = find_attacks_2(5, &bishops);

        // Assert
        assert_eq!(attacks, 2);
    }
}
