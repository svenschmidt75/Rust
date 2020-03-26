// Cracking the Coding Interview,
// 6th ed., page 70

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn find_permutations<'a>(s: &str, b: &'a str) -> Vec<&'a str> {
    assert!(s.len() < b.len());

    // SS: find all permutations os smaller string s in string b

    // SS: rather than generating all permutations of s and then finding them in b,
    // we march through b with a window of size s and check whether all characters
    // are a permutation of s.
    let mut hash = HashMap::new();
    s.chars().for_each(|c| match hash.get_mut(&c) {
        None => {
            hash.insert(c, 1);
        }
        Some(v) => {
            *v += 1;
        }
    });

    let mut results = vec![];

    // SS: runtime: O(b * s), memory O(s)
    let bc: Vec<char> = b.chars().collect();
    for i in 0..(bc.len() - s.len() + 1) {
        // SS: check up to the next s.len() chars in b...
        let substr = &b[i..(i + s.len())];
        if is_permutation(substr, &hash) {
            results.push(substr);
        }
    }

    results
}

fn is_permutation(a: &str, frequency_map: &HashMap<char, u32>) -> bool {
    let mut current_hash = frequency_map.clone();
    for c in a.chars() {
        match current_hash.get_mut(&c) {
            None => {
                return false;
            }
            Some(entry) => {
                if *entry == 0 {
                    return false;
                }
                *entry -= 1;
            }
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::find_permutations;

    #[test]
    fn test1() {
        // Arrange
        let s = "abbc";
        let b = "cbabadcbbabbcbabaabccbabc";

        // Act
        let results = find_permutations(s, b);

        // Act
        assert_eq!(results.len(), 7);
    }
}
