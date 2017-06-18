extern crate image;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};

const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];

pub mod model;
pub mod geometry;
pub mod renderer;


fn main() {
    let mut imgbuf = ImageBuffer::new(100, 100);
    renderer::line(20, 13, 40, 80, &mut imgbuf, image::Rgb(WHITE));
    let v0 = geometry::Vertex3 { x: 1.0, y: 1.0, z: 1.0 };
    println!("{:?}", v0);
    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
