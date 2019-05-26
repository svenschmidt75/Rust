use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read, Result};
use std::path::Path;

use byteorder::{BigEndian, ByteOrder};

use crate::image::Image;
use crate::labels::Label;

const PROJECT_DIRECTORY: &'static str = "/home/svenschmidt75/Develop/Rust/NeuralNetwork/lib/mnist/mnist_loader/";

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let size = std::mem::align_of::<u32>();
    if data.len() < offset + size {
        return None;
    }
    let value = BigEndian::read_u32(&data[offset..offset + size]);
    Some(value)
}

fn read_image(data: &[u8], offset: usize, nrows: usize, ncols: usize) -> Option<Image> {
    let size = nrows * ncols;
    if data.len() < offset + size {
        return None;
    }
    let image = Image {
        data: Vec::from(&data[offset..offset + size]),
        width: ncols,
        height: nrows,
    };
    Some(image)
}

fn read_images(data: &[u8], num_images: usize, offset: usize, nrows: usize, ncols: usize) -> Option<Vec<Image>> {
    let mut images = Vec::<Image>::with_capacity(num_images);
    let end = data.len();
    let mut current_offset = offset;
    while current_offset < end {
        let image = read_image(data, current_offset, nrows, ncols)?;
        images.push(image);
        current_offset += nrows * ncols;
    }
    Some(images)
}

fn read_labels(data: &[u8], num_labels: usize, offset: usize) -> Option<Vec<Label>> {
    let labels = (0..num_labels).map(|idx| Label { label: data[offset + idx] }).collect();
    Some(labels)
}

fn parse_image_data(data: &[u8]) -> Option<Vec<Image>> {
    let magic_number = read_u32(data, 0)?;
    assert_eq!(2051, magic_number);
    let num_images = read_u32(data, 4)? as usize;
    let num_rows = read_u32(data, 8)? as usize;
    let num_cols = read_u32(data, 12)? as usize;
    let images = read_images(data, num_images, 16, num_rows, num_cols)?;
    Some(images)
}

fn parse_label_data(data: &[u8]) -> Option<Vec<Label>> {
    let magic_number = read_u32(data, 0)?;
    assert_eq!(2049, magic_number);
    let num_labels = read_u32(data, 4)? as usize;
    let labels = read_labels(data, num_labels, 8)?;
    Some(labels)
}

pub fn load_image_file(file_name: &str) -> Result<Vec<Image>> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    let n = file.read_to_end(&mut buffer)?;
    assert!(n > 0);
    let images = parse_image_data(&buffer).ok_or(ErrorKind::InvalidData)?;
    Ok(images)
}

fn load_label_file(file_name: &str) -> Result<Vec<Label>> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    let n = file.read_to_end(&mut buffer)?;
    assert!(n > 0);
    let labels = parse_label_data(&buffer).ok_or(ErrorKind::InvalidData)?;
    Ok(labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_train_images() -> Result<()> {
        // Arrange / Act
        let images = load_image_file(&(PROJECT_DIRECTORY.to_owned() + "../../../MNIST/train-images.idx3-ubyte"))?;

        // Assert
        assert_eq!(images.len(), 60000);
        Ok(())
    }

    #[test]
    fn test_read_test_images() -> Result<()> {
        // Arrange / Act
        let images = load_image_file(&(PROJECT_DIRECTORY.to_owned() + "../../../MNIST/t10k-images.idx3-ubyte"))?;

        // Assert
        assert_eq!(images.len(), 10000);
        Ok(())
    }

    #[test]
    fn test_read_train_labes() -> Result<()> {
        // Arrange / Act
        let labels = load_label_file(&(PROJECT_DIRECTORY.to_owned() + "../../../MNIST/train-labels.idx1-ubyte"))?;

        // Assert
        assert_eq!(labels.len(), 60000);
        Ok(())
    }
}
