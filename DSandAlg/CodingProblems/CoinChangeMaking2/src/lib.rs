// https://www.youtube.com/watch?v=sn0DWI-JdNA&list=PLI1t_8YX-ApvMthLj56t1Rf-Buio5Y8KL&index=10

fn make_change(coins: &[u32], amount: u32) -> u32 {
    // SS: Calculate how many ways there are to make `amount` with the given coins.
    make_change_recursive(coins, amount, 0)
}

fn make_change_recursive(coins: &[u32], amount: u32, index: usize) -> u32 {
    // SS: base case
    if index == coins.len() {
        if amount == 0 {
            1
        } else {
            0
        }
    } else {
        let mut n = 0;

        // SS: use coin?
        let coin = coins[index];
        let mut rem = amount;
        while rem >= coin {
            rem -= coin;
            n += make_change_recursive(coins, rem, index + 1);
        }

        // SS: count without this coin
        n += make_change_recursive(coins, amount, index + 1);

        n
    }
}

fn make_change_bottom_up(coins: &[u32], amount: u32) -> u32 {
    // SS: coins are ordered w.r.t. decreasing denomination
    let mut memoization_array = vec![vec![0; coins.len()]; (amount + 1) as usize];

    for a in 1..=amount {
        for c1 in 0..coins.len() {
            let mut n = 0;
            let coin = coins[c1];

            let mut rem = a;
            while rem >= coin {
                rem -= coin;

                // SS: base case
                if rem == 0 {
                    n += 1;
                } else {
                    for c2 in (c1 + 1)..coins.len() {
                        let v = memoization_array[rem as usize][c2];
                        n += v;
                    }
                }
            }

            memoization_array[a as usize][c1] = n;
        }
    }

    let mut n = 0;
    for i in 0..coins.len() {
        n += memoization_array[amount as usize][i];
    }
    n
}


#[cfg(test)]
mod tests {
    use crate::{make_change, make_change_bottom_up};

    #[test]
    fn test1() {
        // Arrange
        let coins = [25, 10, 5, 1];

        // Act
        let n = make_change(&coins, 27);

        // Assert
        assert_eq!(n, 13);
    }

    #[test]
    fn test2() {
        // Arrange
        let coins = [50, 25, 10, 5, 1];

        // Act
        let n = make_change(&coins, 79);

        // Assert
        assert_eq!(n, 134);
    }

    #[test]
    fn test3() {
        // Arrange
        let coins = [1, 2, 3];

        // Act
        let n = make_change(&coins, 5);

        // Assert
        assert_eq!(n, 5);
    }

    #[test]
    fn test1_bottom_up() {
        // Arrange
        let coins = [25, 10, 5, 1];

        // Act
        let n = make_change(&coins, 27);

        // Assert
        assert_eq!(n, 13);
    }

    #[test]
    fn test2_bottom_up() {
        // Arrange
        let coins = [50, 25, 10, 5, 1];

        // Act
        let n = make_change(&coins, 79);

        // Assert
        assert_eq!(n, 134);
    }

    #[test]
    fn test3_bottom_up() {
        // Arrange
        let coins = [1, 2, 3];

        // Act
        let n = make_change_bottom_up(&coins, 9);

        // Assert
        assert_eq!(n, 5);
    }

}
