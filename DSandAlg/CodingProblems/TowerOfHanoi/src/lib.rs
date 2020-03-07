#![feature(test)]

extern crate test;

use std::collections::VecDeque;

// SS: we are using stacks for the three pegs

fn hanoi(n: u8, from: &mut VecDeque<u8>, to: &mut VecDeque<u8>, tmp: &mut VecDeque<u8>) {
    // SS: runtime complexity is O(2^n)

    // SS: base case
    if n == 1 {
        // SS: move disc from 'from' to 'to'
        let disc = from.pop_back().unwrap();
        to.push_back(disc);
    } else {
        // SS: move tower n-1 from 'from' to 'tmp', using 'to' as temp
        hanoi(n - 1, from, tmp, to);

        // SS: move disc from 'from' to 'to'
        let disc = from.pop_back().unwrap();
        to.push_back(disc);

        // SS: move tower n-1 from 'tmp' to 'to', using 'from' as temp
        hanoi(n - 1, tmp, to, from);
    }
}

fn hanoi_with_steps(n: u8, from: &str, to: &str, tmp: &str) {
    // SS: base case
    if n == 1 {
        // SS: move disc from 'from' to 'to'
        println!("moving disc from {} to {}", from, to);
    } else {
        // SS: move tower n-1 from 'from' to 'tmp', using 'to' as temp
        hanoi_with_steps(n - 1, from, tmp, to);

        // SS: move disc from 'from' to 'to'
        println!("moving disc from {} to {}", from, to);

        // SS: move tower n-1 from 'tmp' to 'to', using 'from' as temp
        hanoi_with_steps(n - 1, tmp, to, from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use test::Bencher;

    #[test]
    fn it_works() {
        // Arrange
        let n = 10;

        let mut from = VecDeque::new();
        let mut to = VecDeque::new();
        let mut tmp = VecDeque::new();

        // SS: insert discs
        for i in 0..n {
            from.push_back(n - i);
        }

        // Act
        hanoi(n, &mut from, &mut to, &mut tmp);

        // Assert
        for i in 0..n {
            let disc = to.pop_back().unwrap();
            assert_eq!(disc, i + 1);
        }
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let n = 10;

            let mut from = VecDeque::new();
            let mut to = VecDeque::new();
            let mut tmp = VecDeque::new();

            // SS: insert discs
            for i in 0..n {
                from.push_back(n - i);
            }

            // Act
            hanoi(n, &mut from, &mut to, &mut tmp);
        });
    }

    #[test]
    fn hanoi_with_steps_test() {
        // Arrange
        let n = 4;

        // Act
        hanoi_with_steps(n, "A", "C", "B");

        // Assert
    }
}
