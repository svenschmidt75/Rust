// https://www.reddit.com/r/CodingProblems/comments/fdcsd6/day_82020034_problem_of_the_day_asked_by_google/

use std::cmp;

fn solution(heights: &[u32], nropes: u32, nbricks: u32, building_index: usize) -> usize {
    // SS: Divide and Conquer
    // This problem has overlapping sub-problems, so could use DP to decrease the runtime
    // from O(2^{nropes + nbricks}) to O(nropes * nbricks)...

    // SS: base conditions
    if building_index == heights.len() - 1 {
        building_index
    } else {
        let current_height = heights[building_index];
        let next_height = heights[building_index + 1];

        if current_height < next_height {
            let delta_height = next_height - current_height;

            let mut c1 = 0;
            if nropes > 0 {
                c1 = solution(heights, nropes - 1, nbricks, building_index + 1);
            }

            let mut c2 = 0;
            if nbricks >= delta_height {
                c2 = solution(heights, nropes, nbricks - delta_height, building_index + 1);
            }

            cmp::max(c1, cmp::max(c2, building_index))
        } else {
            // SS: next building is smaller, no ropes or bricks needed
            solution(heights, nropes, nbricks, building_index + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test1() {
        // Arrange
        let heights = [4, 2, 7, 6, 9, 11, 14, 12, 8];

        // Act
        let max_building = solution(&heights, 2, 5, 0);

        // Assert
        assert_eq!(max_building, 8);
    }

    #[test]
    fn test2() {
        // Arrange
        let heights = [4, 2, 7, 6, 9, 11, 14, 12, 8];

        // Act
        let max_building = solution(&heights, 1, 5, 0);

        // Assert
        assert_eq!(max_building, 5);
    }
}
