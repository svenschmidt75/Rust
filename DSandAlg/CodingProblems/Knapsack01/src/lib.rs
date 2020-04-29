use std::cmp;

fn knapsack(weights: &[u32], profits: &[u32], knapsack_capacity: i32, item_index: usize) -> u32 {
    // SS: Divide & Conquor approach
    if knapsack_capacity <= 0 || item_index < 0 || item_index == weights.len() {
        0
    } else {
        let mut profit1 = 0;

        // SS: does the current item fit?
        let current_item_weight = weights[item_index];
        if current_item_weight <= knapsack_capacity as u32 {
            // SS: yes, put it in the knapsack and check other items for
            // remaining capacity...
            profit1 = profits[item_index]
                + knapsack(
                    weights,
                    profits,
                    knapsack_capacity - current_item_weight as i32,
                    item_index + 1,
                );
        }

        // SS: current item does not fit, skip
        let profit2 = knapsack(weights, profits, knapsack_capacity, item_index + 1);

        cmp::max(profit1, profit2)
    }
}

#[cfg(test)]
mod tests {
    use crate::knapsack;

    #[test]
    fn test1() {
        // SS: Udemy, Data Structures and Algorithms, S12.4 - L08

        // Arrange
        let weights = [3, 1, 5, 2];
        let profits = [31, 26, 72, 17];

        // Act
        let max_profit = knapsack(&weights, &profits, 7, 0);

        // Assert
        assert_eq!(max_profit, 98);
    }

    #[test]
    fn test2() {
        // SS: Grokking Algorithms, Manning, Aditya Y. Bhargava

        // Arrange
        let weights = [1, 3, 4];
        let profits = [1500, 2000, 3000];

        // Act
        let max_profit = knapsack(&weights, &profits, 4, 0);

        // Assert
        assert_eq!(max_profit, 3500);
    }
}
