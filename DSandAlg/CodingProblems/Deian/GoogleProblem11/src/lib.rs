/* Implement a class with the following methods:
    - Add()
    - Remove()
    - GetRandomElement()
   Duplicate entries should be ignored.
*/

use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct MyType<'a> {
    hash: HashMap<u64, (&'a str, usize)>,
    array: Vec<u64>,
    random: ThreadRng,
}

impl<'a> MyType<'a> {
    fn new() -> MyType<'a> {
        MyType {
            hash: HashMap::new(),
            array: vec![],
            random: rand::thread_rng(),
        }
    }

    fn put(&mut self, value: &'a str) {
        // SS: vector push has runtime amortized O(1), total runtime is O(1)

        // SS: hash string
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash_value = hasher.finish();

        if self.hash.contains_key(&hash_value) == false {
            let pos = self.array.len();
            self.array.push(hash_value);
            self.hash.insert(hash_value, (value, pos));
        }
    }

    fn remove(&mut self, value: &str) {
        // SS: remove from hash map is O(1)
        // resizing vector is O(1)

        // SS: hash string
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash_value = hasher.finish();

        if let Some((_, pos)) = self.hash.remove(&hash_value) {
            // SS: swap with last item as order doesn't matter
            let other_pos = self.array.len() - 1;
            if pos != other_pos {
                self.array.swap(pos, other_pos);
                let other_hash = self.array[pos];
                let (_, other_array_pos) = self.hash.get_mut(&other_hash).unwrap();
                *other_array_pos = pos;
            }
            self.array.remove(other_pos);
        }
    }

    fn get_random(&mut self) -> &'a str {
        // SS: O(1)
        let index: usize = self.random.gen_range(0, self.array.len());
        let hash_value = self.array[index];
        let (item, _) = *(self.hash.get(&hash_value).unwrap());
        item
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_put() {
        // Arrange
        let mut mytype = MyType::new();

        // Act
        mytype.put("test");

        // Act
    }

    #[test]
    fn test_remove() {
        // Arrange
        let mut mytype = MyType::new();
        mytype.put("test1");
        mytype.put("test2");
        mytype.put("test3");

        // Act
        mytype.remove("test2");

        // Act
    }

    #[test]
    fn test_get_random() {
        // Arrange
        let mut mytype = MyType::new();
        mytype.put("test1");
        mytype.put("test2");
        mytype.put("test3");

        // Act
        let item = mytype.get_random();

        // Act
    }
}
