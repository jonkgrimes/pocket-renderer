extern crate image;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};

const RED: [u8; 3] = [255 as u8, 0 as u8, 0 as u8];
const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];
const BLUE: [u8; 3] = [0 as u8, 0 as u8, 255 as u8];

pub mod renderer;

fn main() {
    let mut imgbuf = ImageBuffer::new(100, 100);
    renderer::line(13, 20, 80, 40, &mut imgbuf, image::Rgb(RED));
    // renderer::line(20, 13, 40, 80, &mut imgbuf, image::Rgb(WHITE));
    // renderer::line(1, 13, 42, 73, &mut imgbuf, image::Rgb(BLUE));
    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
