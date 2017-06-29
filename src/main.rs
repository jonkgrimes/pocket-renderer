extern crate image;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};

use model::Model;
use geometry::Vertex3;

const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub mod model;
pub mod geometry;
pub mod renderer;

fn main() {
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1); // +1 hack to get over the out of bounds errors
    let model = Model::new("models/african_head.obj");

    println!("verts = {}, faces = {}", model.verts_len(), model.faces_len());

    for face in model.faces {
        for index in 0..3 {
            let v0_index = *face.get(index).unwrap();
            let v0: &Vertex3<f32> = model.verts.get(v0_index as usize).unwrap();
            let v1_index = *face.get((index + 1) % 3).unwrap();
            let v1: &Vertex3<f32> = model.verts.get(v1_index as usize).unwrap();
            let x0 = (v0.x + 1.0) * (WIDTH as f32 / 2.0);
            let y0 = (v0.y + 1.0) * (HEIGHT as f32 / 2.0);
            let x1 = (v1.x + 1.0) * (WIDTH as f32 / 2.0);
            let y1 = (v1.y + 1.0) * (HEIGHT as f32 / 2.0);
            if (x0 as i32 - x1 as i32) == (y0 as i32 - y1 as i32) {
                println!("Perfect line");
                println!("{:?}",v0);
                println!("{:?}",v1);
                println!("x0 = {}", x0);
                println!("x0 as i32 = {}", x0 as i32);
                println!("y0 = {}", y0);
                println!("y0 as i32 = {}", y0 as i32);
                println!("x1 = {}", x1);
                println!("x1 as i32 = {}", x1 as i32);
                println!("y1 = {}", y1);
                println!("y1 as i32 = {}", y1 as i32);
            } else {
                renderer::line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut imgbuf, image::Rgb(WHITE));
            }
        }
    }

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
