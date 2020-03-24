// https://www.reddit.com/r/CodingProblems/comments/fcdmg5/day_72020032_problem_of_the_day_asked_by/

use std::cmp;

fn calc_max_profit(input: &[(u32, u32)]) -> u32 {
    // SS: Use dynamic programming to maximize the profit from AirBnB bookings
    let mut max_profit = 0;

    for row in 0..input.len() {
        let mut row_profit = 0;
        let current_booking = input[row];
        for col in 0..row {
            let booking = input[col];
            if booking.1 < current_booking.0 {
                // SS: no conflicts
                let days = booking.1 - booking.0 + 1;
                row_profit += days;
            }
        }

        // SS: add current booking
        let days = current_booking.1 - current_booking.0 + 1;
        row_profit += days;
        max_profit = cmp::max(max_profit, row_profit);
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use crate::calc_max_profit;

    #[test]
    fn test1() {
        // Arrange
        let bookings = [(1, 2), (4, 5), (7, 7)];

        // Act
        let profit = calc_max_profit(&bookings);

        // Assert
        assert_eq!(5, profit);
    }

    #[test]
    fn test2() {
        // Arrange
        let bookings = [(4, 5), (7, 9), (1, 100)];

        // Act
        let profit = calc_max_profit(&bookings);

        // Assert
        assert_eq!(100, profit);
    }

    #[test]
    fn test3() {
        // Arrange
        let bookings = [(5, 17), (7, 53), (54, 60)];

        // Act
        let profit = calc_max_profit(&bookings);

        // Assert
        assert_eq!(67, profit);
    }

    #[test]
    fn test4() {
        // Arrange
        let bookings = [(5, 17), (7, 54), (54, 60)];

        // Act
        let profit = calc_max_profit(&bookings);

        // Assert
        assert_eq!(48, profit);
    }
}
