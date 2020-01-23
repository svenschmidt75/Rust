fn buy_sell_stock_slow(stock_price: &[i64]) -> Option<(usize, usize, i64)> {
    // SS: runtime is O(N^2), at O(1) space complexity
    let mut best_buy = 0;
    let mut best_sell = 0;
    let mut best_profit = 0;

    for i in 0..stock_price.len() {
        let buy_price = stock_price[i];

        // SS: solve subproblem optimally
        for j in (i + 1)..stock_price.len() {
            let sell_price = stock_price[j];
            let profit = sell_price - buy_price;
            if profit > best_profit {
                best_buy = i;
                best_sell = j;
                best_profit = profit;
            }
        }
    }

    if best_profit > 0 {
        Some((best_buy, best_sell, best_profit))
    } else {
        None
    }
}

fn buy_sell_stock_fast(stock_price: &[i64]) -> Option<i64> {
    // SS: runtime is O(N), at O(1) space complexity
    if stock_price.is_empty() {
        return None;
    }

    let mut min_stock_price = stock_price[0];
    let mut best_profit = 0;

    for i in 1..stock_price.len() {
        let sell_price = stock_price[i];
        let profit = sell_price - min_stock_price;
        if profit > best_profit {
            best_profit = profit;
        }

        if sell_price < min_stock_price {
            min_stock_price = sell_price;
        }
    }

    Some(best_profit)
}

#[cfg(test)]
mod tests {
    use crate::{buy_sell_stock_fast, buy_sell_stock_slow};

    #[test]
    fn test1_slow() {
        // Arrange
        let stock_price = [7, 1, 5, 3, 6, 4];

        // Act
        let (best_buy, best_sell, best_profit) = buy_sell_stock_slow(&stock_price).unwrap();

        // Assert
        assert_eq!(best_profit, 5);
        assert_eq!(best_buy, 1);
        assert_eq!(best_sell, 4);
    }

    #[test]
    fn test1_fast() {
        // Arrange
        let stock_price = [7, 1, 5, 3, 6, 4];

        // Act
        let best_profit = buy_sell_stock_fast(&stock_price).unwrap();

        // Assert
        assert_eq!(best_profit, 5);
    }

    #[test]
    fn test2_slow() {
        // Arrange
        let stock_price = [7, 6, 4, 3, 1];

        // Act
        let best_profit = buy_sell_stock_slow(&stock_price);

        // Assert
        assert_eq!(best_profit, None);
    }

    #[test]
    fn test2_fast() {
        // Arrange
        let stock_price = [7, 6, 4, 3, 1];

        // Act
        let best_profit = buy_sell_stock_fast(&stock_price).unwrap();

        // Assert
        assert_eq!(best_profit, 0);
    }
}
