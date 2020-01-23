fn climb_stairs(n: i64) -> u64 {
    // SS: terrible runtime performance, O(2^n), due to the two branches
    if n == 0 {
        0
    } else {
        let mut solutions = 0;
        climb_stairs_internal(n, &mut solutions);
        solutions
    }
}

fn climb_stairs_internal(n: i64, solutions: &mut u64) {
    if n == 0 {
        *solutions += 1;
    } else if n > 0 {
        climb_stairs_internal(n - 2, solutions);
        climb_stairs_internal(n - 1, solutions);
    }

    // SS: n < 0 not a solution
}

fn climb_stairs_memoize(n: i64) -> u64 {
    // SS: terrible runtime performance, O(2^n), due to the two branches
    if n == 0 {
        0
    } else {
        let mut memoize = vec![0; n as usize + 1];
        climb_stairs_memoize_internal(n, &mut memoize);
        memoize[n as usize]
    }
}

fn climb_stairs_memoize_internal(n: i64, memoize: &mut [u64]) -> u64 {
    if n == 0 {
        // SS: solution found
        1
    } else if n > 0 {
        if memoize[n as usize] == 0 {
            let n1 = climb_stairs_memoize_internal(n - 1, memoize);
            let n2 = climb_stairs_memoize_internal(n - 2, memoize);
            memoize[n as usize] = n1 + n2;
        }
        memoize[n as usize]
    } else {
        // SS: n < 0 not a solution
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{climb_stairs, climb_stairs_memoize};

    #[test]
    fn test1() {
        // Arrange
        let nstairs = 2;

        // Act
        let solutions = climb_stairs(nstairs);

        // Assert
        assert_eq!(2, solutions);
    }

    #[test]
    fn test2() {
        // Arrange
        let nstairs = 3;

        // Act
        let solutions = climb_stairs(nstairs);

        // Assert
        assert_eq!(3, solutions);
    }

    #[test]
    fn test1_memoize() {
        // Arrange
        let nstairs = 2;

        // Act
        let solutions = climb_stairs_memoize(nstairs);

        // Assert
        assert_eq!(2, solutions);
    }

    #[test]
    fn test2_memoize() {
        // Arrange
        let nstairs = 3;

        // Act
        let solutions = climb_stairs_memoize(nstairs);
        // Assert
        assert_eq!(3, solutions);
    }
}
