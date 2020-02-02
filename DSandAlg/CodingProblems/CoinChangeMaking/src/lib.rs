// https://leetcode.com/problems/coin-change/

use std::cmp;

fn make_change(coins: &[u8], amount: u8) -> u8 {
    // SS: assume coins are ordered by denominations
    let mut grid = vec![0; amount as usize + 1];

    for current_amount in 1..=amount {
        let mut ncoins_current_amount = std::u8::MAX;

        for &coin in coins {
            // SS: divide amount by coin
            let m = current_amount / coin;
            if m == 0 {
                // SS: coin does not divide amount
                break;
            }

            let mut ncoins = m;

            // SS: The minimum number of coins for the remainder has already been
            // calculated. They are the subproblems for this DP problem
            let r = current_amount % coin;
            if r > 0 {
                // SS: optimal subproblem solution
                let p = grid[r as usize];
                if p == std::u8::MAX {
                    break;
                }
                ncoins += p;
            }

            ncoins_current_amount = cmp::min(ncoins_current_amount, ncoins);
        }

        grid[current_amount as usize] = ncoins_current_amount;
    }

    let ncoins = grid[grid.len() - 1];
    if ncoins == std::u8::MAX {
        // SS: no solution
        0
    } else {
        ncoins
    }
}

#[cfg(test)]
mod tests {
    use crate::make_change;

    #[test]
    fn test1() {
        // Arrange
        let coins = [1, 5, 12, 19];

        // Act
        let result = make_change(&coins, 16);

        // Act
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        // Arrange
        let coins = [1, 2, 5];

        // Act
        let result = make_change(&coins, 11);

        // Act
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        // Arrange
        let coins = [2];

        // Act
        let result = make_change(&coins, 3);

        // Act
        assert_eq!(result, 0);
    }
}
