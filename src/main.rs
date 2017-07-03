extern crate image;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};

use geometry::Vertex2;

const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];
const RED: [u8; 3] = [255 as u8, 0 as u8, 0 as u8];
const GREEN: [u8; 3] = [0 as u8, 255 as u8, 0 as u8];
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub mod model;
pub mod geometry;
pub mod renderer;

fn main() {
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT); // +1 hack to get over the out of bounds errors

    let t0: [Vertex2<i32>; 3] = [Vertex2::<i32>::new(10, 70), Vertex2::<i32>::new(50, 160), Vertex2::<i32>::new(70, 80)];
    let t1: [Vertex2<i32>; 3] = [Vertex2::<i32>::new(180, 50), Vertex2::<i32>::new(150, 1), Vertex2::<i32>::new(70, 180)];
    let t2: [Vertex2<i32>; 3] = [Vertex2::<i32>::new(180, 150), Vertex2::<i32>::new(120, 160), Vertex2::<i32>::new(130, 180)];

    renderer::triangle(&t0[0], &t0[1], &t0[2], &mut imgbuf, image::Rgb(WHITE));
    renderer::triangle(&t1[0], &t1[1], &t1[2], &mut imgbuf, image::Rgb(GREEN));
    renderer::triangle(&t2[0], &t2[1], &t2[2], &mut imgbuf, image::Rgb(RED));

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
