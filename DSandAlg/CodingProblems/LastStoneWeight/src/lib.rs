// https://leetcode.com/problems/last-stone-weight/

use crate::pq::PriorityQueue;

mod pq;

fn find_stone_weight(stone_weights: &[u32]) -> u32 {
    if stone_weights.is_empty() {
        0
    } else {
        let mut pq = PriorityQueue::new();
        stone_weights.iter().for_each(|&weight| {
            pq.insert(weight as i64, weight);
        });

        loop {
            if pq.is_empty() {
                return 0;
            }

            let weight1 = pq.pop().1;
            if pq.is_empty() {
                return weight1;
            }

            let weight2 = pq.pop().1;

            if weight1 == weight2 {
                // SS: both stones destroyed
                continue;
            }

            // SS: stones have unequal weight
            let new_weight = weight1 - weight2;
            pq.insert(new_weight as i64, new_weight);
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::find_stone_weight;

    #[test]
    fn test1() {
        // Arrange
        let stones = [2, 7, 4, 1, 8, 1];

        // Act
        let last_stone_weight = find_stone_weight(&stones);

        // Assert
        assert_eq!(last_stone_weight, 1);
    }

    #[test]
    fn test_empty() {
        // Arrange
        let stones = [];

        // Act
        let last_stone_weight = find_stone_weight(&stones);

        // Assert
        assert_eq!(last_stone_weight, 0);
    }
}
