extern crate image;
extern crate rand;

use std::fs::File;
use std::path::Path;
use image::ImageBuffer;
use model::Model;
use geometry::Vertex2;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub mod model;
pub mod geometry;
pub mod renderer;

fn main() {
    // +1 hack to get over the out of bounds errors
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1);
    let model = Model::new("models/african_head.obj");

    for face in model.faces {
        let mut screen_coords: [Vertex2<i32>; 3] = [
            Vertex2::<i32> {x: 0, y: 0},
            Vertex2::<i32> {x: 0, y: 0},
            Vertex2::<i32> {x: 0, y: 0},
        ];
        for i in 0..3 {
            let vertex_index = *face.get(i).unwrap() as usize;
            let world_coords = model.verts.get(vertex_index).unwrap();
            screen_coords[i] = Vertex2::<i32> {
                x: ((world_coords.x + 1.0) * WIDTH as f32 / 2.0) as i32,
                y: ((world_coords.y + 1.0) * HEIGHT as f32 / 2.0) as i32
            } ;
        }
        let pixel = image::Rgb([rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()]);
        renderer::triangle(&screen_coords[0], &screen_coords[1], &screen_coords[2], &mut imgbuf, pixel);
    }

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(image::imageops::flip_vertical(&imgbuf)).save(fout, image::PNG);
}
