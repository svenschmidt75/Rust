// Maximum Rectangular Area in Histogram
// Tushar Roy
// https://www.youtube.com/watch?v=ZmnqCZp9bBs

use std::cmp;
use std::collections::VecDeque;

fn max_area(heights: &[u32]) -> u32 {
    // SS: partition heights by 0 and do each partition separately

    let mut max_height = 0;

    let mut pos = 0;
    let mut start = 0;
    while pos < heights.len() {
        while pos < heights.len() && heights[pos] == 0 {
            pos += 1;
        }

        start = pos;
        while pos < heights.len() && heights[pos] > 0 {
            pos += 1;
        }

        let height = max_area2(heights, start, pos - 1);
        max_height = cmp::max(max_height, height);
    }
    max_height
}

fn max_area2(heights: &[u32], min: usize, max: usize) -> u32 {
    // SS: runtime is O(max height * n), n = max - min + 1
    let mut max_height = 0;

    let mut height = 1;
    let mut changed = false;
    loop {
        let mut pos = min;
        while pos <= max {
            if heights[pos] < height {
                pos += 1;
                continue;
            }

            let start = pos;
            while pos <= max && heights[pos] >= height {
                max_height = cmp::max(max_height, heights[pos]);
                pos += 1;
            }

            let width = (pos - start) as u32;
            if width > 0 {
                max_height = cmp::max(max_height, width * height);
                changed = true;
            }
        }

        if changed == false {
            break;
        }

        changed = false;
        height += 1;
    }

    max_height
}

fn area_simple(heights: &[u32]) -> u32 {
    if heights.is_empty() {
        0
    } else {
        let mut stack = VecDeque::new();

        let mut max_area = 0;
        let mut area = 0;
        let mut i = 0;

        while i < heights.len() {
            if stack.is_empty() || heights[*stack.back().unwrap() as usize] <= heights[i] {
                stack.push_back(i);
                i += 1;
            } else {
                // SS: calculate area
                let top = stack.pop_back().unwrap();
                if stack.is_empty() {
                    area = heights[top] * i as u32;
                } else {
                    area = heights[top] * (i - *stack.back().unwrap() as usize - 1) as u32;
                }
                max_area = cmp::max(max_area, area);
            }
        }

        while stack.is_empty() == false {
            let top = stack.pop_back().unwrap();
            if stack.is_empty() {
                area = heights[top] * (i as u32);
            } else {
                area = heights[top] * (i - *stack.back().unwrap() as usize - 1) as u32;
            }
            max_area = cmp::max(max_area, area);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::{area_simple, max_area};

    #[test]
    fn test1() {
        // Arrange
        let heights = [1, 2, 0, 3];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 3);
    }

    #[test]
    fn test2() {
        // Arrange
        let heights = [1, 2, 4];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 4);
    }

    #[test]
    fn test3() {
        // Arrange
        let heights = [1, 1, 2, 2];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 4);
    }

    #[test]
    fn test4() {
        // Arrange
        let heights = [1, 2, 1];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 3);
    }

    #[test]
    fn test51() {
        // Arrange
        let heights = [1, 2, 3, 2];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 6);
    }

    #[test]
    fn test52() {
        // Arrange
        let heights = [1, 2, 3, 2];

        // Act
        let max_area = area_simple(&heights);

        // Assert
        assert_eq!(max_area, 6);
    }

    #[test]
    fn test6() {
        // Arrange
        let heights = [2, 1, 2, 3, 1];

        // Act
        let max_area = max_area(&heights);

        // Assert
        assert_eq!(max_area, 5);
    }

    #[test]
    fn test72() {
        // Arrange
        let heights = [2, 1, 2, 3, 1];

        // Act
        let max_area = area_simple(&heights);

        // Assert
        assert_eq!(max_area, 5);
    }
}
