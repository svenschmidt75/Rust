use std::cmp;
use std::collections::VecDeque;

struct RunningProduct {
    k: usize,
    buffer: VecDeque<i64>,
    position: usize,
    product: i64,
    time_to_remove_zero: usize,
}

impl RunningProduct {
    fn new(k: usize) -> RunningProduct {
        RunningProduct {
            k,
            buffer: VecDeque::new(),
            position: 0,
            product: 1,
            time_to_remove_zero: 0,
        }
    }

    fn add(&mut self, value: i64) {
        if value == 0 {
            self.time_to_remove_zero = self.k;
        } else {
            if self.time_to_remove_zero > 0 {
                self.time_to_remove_zero = cmp::max(0, self.time_to_remove_zero - 1);
            }
        }

        if self.position < self.k {
            self.buffer.push_back(value);
            self.position += 1;
            self.product *= value;
        } else {
            // SS: overflow
            let overflow_element = self.buffer[0];

            // SS: using a queue or a circular array can avoid this O(N) operation,
            // shifting the array by one...
            self.buffer.pop_front();
            self.buffer.push_back(value);

            self.product = if overflow_element != 0 {
                self.product /= overflow_element;
                self.product *= value;
                self.product
            } else {
                if self.time_to_remove_zero == 0 {
                    let mut product = 1;
                    for item in self.buffer.iter() {
                        product *= *item;
                    }
                    product
                } else {
                    0
                }
            };
        }
    }

    fn get(&self) -> i64 {
        self.product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_zero() {
        // Arrange
        let mut rp = RunningProduct::new(5);
        rp.add(3);
        rp.add(4);
        rp.add(5);
        rp.add(6);
        rp.add(7);

        // Act
        rp.add(8);

        // Assert
        assert_eq!(rp.get(), 4 * 5 * 6 * 7 * 8);
    }

    #[test]
    fn test_zero() {
        // Arrange
        let mut rp = RunningProduct::new(5);
        rp.add(0);
        rp.add(4);
        rp.add(5);
        rp.add(6);
        rp.add(7);

        // Act
        rp.add(8);

        // Assert
        assert_eq!(rp.get(), 4 * 5 * 6 * 7 * 8);
    }

    #[test]
    fn test_zero2() {
        // Arrange
        let mut rp = RunningProduct::new(4);
        rp.add(0);
        rp.add(1);
        rp.add(2);
        rp.add(3);

        // Act
        rp.add(3);

        // Assert
        assert_eq!(rp.get(), 1 * 2 * 3 * 3);
    }
}
