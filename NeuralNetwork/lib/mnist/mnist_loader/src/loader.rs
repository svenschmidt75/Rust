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

//he trait `std::convert::From<std::option::NoneError>` is not implemented for `std::io::Error`

fn parse_image_data(data: &Vec<u8>) -> Result<Vec<Image>> {
    let magic_number = get_magic_number(data).ok_or(ErrorKind::InvalidData)?;
    match magic_number {
        2051 => {}
        _ => return Err(std::io::Error::from(ErrorKind::InvalidData))
    }

    let images = Vec::<Image>::new();
    Ok(images)
}

fn load_image_file(file_name: &str) -> Result<Vec<Image>> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    let n= file.read_to_end(&mut buffer)?;
    assert!(n > 0);
    let images = parse_image_data(&buffer)?;
    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        // Arrange / Act
        let images = load_image_file(&(project_directory.to_owned() + "../../../MNIST/train-images.idx3-ubyte"))?;

        // Assert
        assert_eq!(2 + 2, 4);
        Ok(())
    }
}
