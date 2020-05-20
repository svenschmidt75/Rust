// Sliding Window Technique + 4 Questions - Algorithms
// https://www.youtube.com/watch?v=jM2dhDPYMQM

use std::collections::{HashMap, HashSet};

fn subarray_adds_up_to_number(input: &[i32], sum: i32) {
    // SS: use sliding window technique
    // runtime: O(n)
    let mut i = 0;
    let mut j = 0;
    let mut running_sum = 0;
    while i < input.len() && j < input.len() {
        if running_sum + input[j] < sum {
            running_sum += input[j];
            j += 1;
        } else if running_sum + input[j] > sum {
            running_sum -= input[i];
            i += 1;
        } else {
            println!("Found subarray: {} - {}", i, j);
            j += 1;
            i = j;
            running_sum = 0;
        }
    }
}

fn find_max_sum_subarray(input: &[i32], length: usize) {
    if input.len() < 2 {
        println!("no solution");
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut min = 0;
        let mut max = 0;
        let mut running_sum = 0;
        let mut max_sum = i32::MIN;

        while j < length {
            running_sum += input[j];
            j += 1;
        }

        while j < input.len() {
            if running_sum > max_sum {
                max_sum = running_sum;
                min = i;
                max = j;
            }
            running_sum -= input[i];
            i += 1;
            running_sum += input[j];
            j += 1;
        }

        // SS: for the remainder
        if running_sum > max_sum {
            max_sum = running_sum;
            min = i;
            max = j;
        }

        println!(
            "Largest sum of length {}: [{}, {}); {}",
            length, min, max, max_sum
        );
    }
}

fn max_sequence_with_flipping(input: &[u8], max_flips: u8) {
    if input.len() < 1 {
        println!("no solution");
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut min = 0;
        let mut max = 0;
        let mut max_width = 0;
        let mut flips_remaining = max_flips;

        while i < input.len() && j < input.len() {
            if input[j] == 0 {
                // SS: need to flip 0 to 1
                if flips_remaining > 0 {
                    flips_remaining -= 1;
                    j += 1;
                } else {
                    // SS: no more flips remaining, advance window
                    if j - i > max_width {
                        min = i;
                        max = j;
                        max_width = j - i;
                    }

                    // SS: starting from i, skip until the next 0
                    while input[i] != 0 {
                        i += 1;
                    }
                    i += 1;

                    flips_remaining += 1;
                }
            } else {
                // SS: 1, do advance window
                j += 1;
            }
        }

        // SS: remainder
        if j - i > max_width {
            min = i;
            max = j;
            max_width = j - i;
        }

        println!(
            "Largest subarray: [{}, {}), length: {}",
            min, max, max_width
        );
    }
}

fn max_sequence_with_flipping2(input: &[u8], max_flips: u8) {
    if input.len() < 1 {
        println!("no solution");
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut min = 0;
        let mut max = 0;
        let mut max_width = 0;
        let mut flips_remaining = max_flips;

        while i < input.len() && j < input.len() {
            if input[j] == 0 {
                // SS: need to flip 0 to 1
                if flips_remaining > 0 {
                    flips_remaining -= 1;
                    j += 1;
                } else {
                    // SS: no more flips remaining, advance window
                    if j - i > max_width {
                        min = i;
                        max = j;
                        max_width = j - i;
                    }

                    if input[i] == 0 {
                        flips_remaining += 1;
                    }

                    i += 1;
                }
            } else {
                // SS: 1, do advance window
                j += 1;
            }
        }

        // SS: remainder
        if j - i > max_width {
            min = i;
            max = j;
            max_width = j - i;
        }

        println!(
            "Largest subarray: [{}, {}), length: {}",
            min, max, max_width
        );
    }
}

fn find_chars_no_repeated_chars(input: &str, chars: &[char]) {
    // SS: Given a string `input and a list of n characters, find the shortest substring that
    // contains all n of them.
    // Assumption: all chars in `chars` are unique.
    if input.len() < chars.len() || input.len() == 0 || chars.len() == 0 {
        println!("no solution");
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut min = 0;
        let mut max = 0;
        let mut max_width = input.len();

        let input_str = input.chars().collect::<Vec<_>>();

        // SS: put all chars in a hash set for efficient lookup
        let chars_hash = chars.iter().collect::<HashSet<_>>();
        let mut chars_seen = HashSet::new();

        // SS: skip all characters until we see the 1st desired one
        while i < input.len() && chars_hash.contains(&input_str[i]) == false {
            i += 1;
        }

        if i == input.len() {
            println!("no solution");
            return;
        }

        j = i;

        while i < input.len() && j < input.len() {
            let c = input_str[j];
            if chars_hash.contains(&c) {
                // SS: can we add it?
                if chars_seen.len() == chars.len() {
                    // SS: advance window from left

                    // SS: remove one of the desired chars
                    chars_seen.remove(&input_str[i]);
                    i += 1;

                    while chars_hash.contains(&input_str[i]) == false {
                        i += 1;
                    }

                    j += 1;
                } else {
                    chars_seen.insert(c);

                    if chars_seen.len() == chars.len() {
                        if j - i + 1 < max_width {
                            min = i;
                            max = j;
                            max_width = j - i + 1;
                        }
                    } else {
                        j += 1;
                    }
                }
            } else {
                j += 1;
            }
        }

        println!(
            "Largest subarray: [{}, {}), length: {}",
            min, max, max_width
        );
    }
}

fn find_chars_with_repeated_chars(input: &str, chars: &[char]) {
    // SS: Given a string `input and a list of n characters, find the shortest substring that
    // contains all n of them.
    // Note that `chars` might have repeated characters.
    if input.len() < chars.len() || input.len() == 0 || chars.len() == 0 {
        println!("no solution");
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut min = 0;
        let mut max = 0;
        let mut max_width = input.len();

        let input_str = input.chars().collect::<Vec<_>>();

        // SS: put all chars in a hash map for efficient lookup
        let mut chars_hash = HashMap::new();
        chars.iter().for_each(|&c| {
            let p = chars_hash.entry(c).or_insert(0);
            *p += 1;
        });

        let mut chars_seen = HashMap::new();

        // SS: skip all characters until we see the 1st desired one
        while i < input.len() && chars_hash.contains_key(&input_str[i]) == false {
            i += 1;
        }

        if i == input.len() {
            println!("no solution");
            return;
        }

        j = i;

        // SS: subarray starts with desired char
        let mut chars_seen_cnt = 0;
        let mut chars_seen_cnt2 = 0;

        while i < input.len() && j < input.len() {
            let c = input_str[j];
            if chars_hash.contains_key(&c) {
                // SS: can we add it?
                if chars_seen_cnt == chars.len() {
                    // SS: advance window from left

                    // SS: remove one of the desired chars
                    while i < j {
                        let c = input_str[i];
                        if let Some(p) = chars_seen.get_mut(&c) {
                            if chars_seen_cnt < chars.len() {
                                break;
                            }
                            if chars_seen_cnt == chars.len() {
                                if j - i + 1 < max_width {
                                    min = i;
                                    max = j;
                                    max_width = j - i + 1;
                                }
                            }
                            let &m = chars_hash.get(&c).unwrap();
                            if *p <= m {
                                chars_seen_cnt -= 1;
                            }
                            *p -= 1;
                            chars_seen_cnt2 -= 1;
                        }
                        i += 1;
                    }

                    j += 1;
                } else {
                    // SS: frequency of this char
                    let &m = chars_hash.get(&c).unwrap();
                    let p = chars_seen.entry(c).or_insert(0);

                    if *p < m {
                        chars_seen_cnt += 1;
                    }

                    chars_seen_cnt2 += 1;

                    *p += 1;

                    if chars_seen_cnt == chars.len() {
                        if j - i + 1 < max_width {
                            min = i;
                            max = j;
                            max_width = j - i + 1;
                        }
                    } else {
                        j += 1;
                    }
                }
            } else {
                j += 1;
            }
        }

        println!(
            "Largest subarray: [{}, {}), length: {}",
            min, max, max_width
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_sum_subarray_test() {
        // Arrange
        let input = [-1, 2, 3, 1, -3, 2];

        // Act
        find_max_sum_subarray(&input, 2);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn subarrays_that_add_up_to_9() {
        // Arrange
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Act
        subarray_adds_up_to_number(&input, 9);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn subarrays_that_add_up_to_7() {
        // Arrange
        let input = [1, 7, 4, 3, 1, 2, 1, 5, 1];

        // Act
        subarray_adds_up_to_number(&input, 7);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn max_sequence_of_contiguous_ones_1() {
        // Arrange
        let input = [0, 1, 0, 1, 0, 0, 1, 1];

        // Act
        max_sequence_with_flipping(&input, 2);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn max_sequence_of_contiguous_ones_2() {
        // Arrange
        let input = [0, 1, 0, 1, 0, 0, 1, 1];

        // Act
        max_sequence_with_flipping2(&input, 2);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_find_chars_1() {
        // Arrange
        let input = "fa4chba4c";
        let chars = ['a', 'b', 'c'];

        // Act
        find_chars_no_repeated_chars(input, &chars);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_find_chars_with_repeated_chars_1() {
        // Arrange
        let input = "fa4chba4c";
        let chars = ['a', 'b', 'c'];

        // Act
        find_chars_with_repeated_chars(input, &chars);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_find_chars_with_repeated_chars_2() {
        // Arrange
        let input = "fa4chba4c";
        let chars = ['a', 'b', 'c', 'a'];

        // Act
        find_chars_with_repeated_chars(input, &chars);

        //        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_find_chars_with_repeated_chars_3() {
        // Arrange
        let input = "badabac";
        let chars = ['a', 'a', 'b', 'c'];

        // Act
        find_chars_with_repeated_chars(input, &chars);

        //        assert_eq!(2 + 2, 4);
    }
}
