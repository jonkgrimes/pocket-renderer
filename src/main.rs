extern crate image;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};

use geometry::Vertex2;

// const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];
const RED: [u8; 3] = [255 as u8, 0 as u8, 0 as u8];
// const GREEN: [u8; 3] = [0 as u8, 255 as u8, 0 as u8];
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;

pub mod model;
pub mod geometry;
pub mod renderer;

fn main() {
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1); // +1 hack to get over the out of bounds errors

    let t0: [Vertex2<i32>; 3] = [Vertex2::<i32>::new(10, 10), Vertex2::<i32>::new(100, 30), Vertex2::<i32>::new(190, 160)];

    renderer::triangle(&t0[0], &t0[1], &t0[2], &mut imgbuf, image::Rgb(RED));

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(image::imageops::flip_vertical(&imgbuf)).save(fout, image::PNG);
}
