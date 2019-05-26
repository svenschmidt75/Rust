use std::fs::File;
use std::io::{ErrorKind, Read, Result};
use std::path::Path;
use byteorder::{ByteOrder, BigEndian};

use crate::image::Image;
use std::env;

const project_directory: &'static str = "/home/svenschmidt75/Develop/Rust/NeuralNetwork/lib/mnist/mnist_loader/";

fn get_magic_number(data: &Vec<u8>) -> Option<u32> {
    if data.len() < 4 {
        return None;
    }
    let magic_number = BigEndian::read_u32(&data[0..4]);
    Some(magic_number)
}

fn get_number_of_images(data: &Vec<u8>) -> Option<u32> {
    if data.len() < 4 {
        return None;
    }
    let value = BigEndian::read_u32(&data[4..8]);
    Some(value)
}

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
    let image = Image { data: Vec::from(&data[offset..offset + size]), width: ncols, height: nrows };
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

fn parse_image_data(data: &Vec<u8>) -> Option<Vec<Image>> {
    let magic_number = read_u32(data, 0)?;
    assert_eq!(2051, magic_number);
    let num_images = read_u32(data, 4)? as usize;
    let num_rows = read_u32(data, 8)? as usize;
    let num_cols = read_u32(data, 12)? as usize;
    let images = read_images(data, num_images, 16, num_rows, num_cols)?;
    Some(images)
}

fn load_image_file(file_name: &str) -> Result<Vec<Image>> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    let n= file.read_to_end(&mut buffer)?;
    assert!(n > 0);
    let images = parse_image_data(&buffer).ok_or(ErrorKind::InvalidData)?;
    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        // Arrange / Act
        let images = load_image_file(&(project_directory.to_owned() + "../../../MNIST/train-images.idx3-ubyte"))?;

        println!("{}", images.len());

        // Assert
        assert_eq!(images.len(), 60000);
        Ok(())
    }
}
