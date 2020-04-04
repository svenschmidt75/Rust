// Given a list of train cart weights, find a plane with the minimum possible capacity
// such that all goods can be transported in 5 days.
// You have to accept the train carts in the order they are given.
// Example: Train cart weights: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
// Days 5, the minimum plane capacity is 14, because
// day 1: (1, 2, 3, 4, 5)
// day 2: (6, 7)
// day 3: 8
// day 4: 9
// day 5: 10

fn find_minimum_plane_capacity_1(weights: &[usize], max_days: usize) -> usize {
    if weights.is_empty() {
        return 0;
    }

    if max_days <= 0 {
        return 0;
    }

    // SS: brute-force solution

    let max_weight = *weights.iter().max().unwrap();

    let mut current_capacity = max_weight;
    loop {
        let mut weight_position = 0;
        for day in 0..max_days {
            let mut total_weight = 0;
            while weight_position < weights.len() {
                let weight = weights[weight_position];
                if total_weight + weight <= current_capacity {
                    total_weight += weight;
                    weight_position += 1;
                } else {
                    total_weight = 0;

                    // SS: capacity exceeded for the day, check next day
                    break;
                }
            }

            // SS: does it fit?
            if weight_position == weights.len() {
                return current_capacity;
            }
        }

        current_capacity += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::find_minimum_plane_capacity_1;

    #[test]
    fn test_brute_force_1() {
        // Arrange
        let weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Act
        let capacity = find_minimum_plane_capacity_1(&weights, 5);

        // Assert
        assert_eq!(capacity, 15);
    }

    #[test]
    fn test_brute_force_2() {
        // Arrange
        let weights = [10, 1, 9, 2, 8, 3, 7, 4, 6, 5];

        // Act
        let capacity = find_minimum_plane_capacity_1(&weights, 5);

        // Assert
        assert_eq!(capacity, 11);
    }
}
