fn load_image_file(path: &str) -> Vec<Image> {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // Arrange / Act
        let images = load_image_file("../MNIST/train-images.idx3-ubyte");

        // Assert
        assert_eq!(2 + 2, 4);
    }
}
