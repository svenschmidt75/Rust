fn array_pairs(array: &[u64]) {
    // SS: nested loop, O(N^2)
    for i in 0..array.len() {
        for j in 0..array.len() {
            println!("[{},{}]", array[i], array[j]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::array_pairs;

    #[test]
    fn it_works() {
        let array = [1, 2, 3, 4, 5];
        array_pairs(&array);
    }
}
