use std::env;
use std::fs::File;
use std::io::{BufWriter, Result};
use std::iter::Iterator;
use std::path::Path;

use png::HasParameters;

use mnist_loader::image::Image;
use mnist_loader::loader::load_image_file;

fn write_png(image: &Image, idx: usize, path: &str) -> Result<()> {
    let image_name = format!("image_{}.png", idx);
    let file_name = Path::new(path).join(image_name);
    let file = File::create(file_name)?;
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image.data)?;
    Ok(())
}

fn main() -> Result<()> {
    let image_file_name = env::args().nth(1).unwrap();
    println!("First argument: {}", image_file_name);
    let dest_folder = env::args().nth(2).unwrap();
    println!("Second argument: {}", dest_folder);
    let images: Vec<Image> = load_image_file(&image_file_name)?;
    images.iter().enumerate().map(|p| {
        println!("Writing image {}/{}", p.0 + 1, images.len());
        write_png(p.1, p.0, &dest_folder)
    }).collect::<Result<()>>()?;
    Ok(())
}
