extern crate image;

use std::fs::File;
use std::path::Path;
use image::ImageBuffer;
use model::Model;
use geometry::Vertex2;
use geometry::Vertex3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub mod model;
pub mod geometry;
pub mod renderer;

fn main() {
    // +1 hack to get over the out of bounds errors
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1);
    let model = Model::new("models/african_head.obj");
    let light_dir = Vertex3::<f32> { x: 0.0, y: 0.0, z: -1.0 };

    for face in model.faces {
        let mut screen_coords: [Vertex2<i32>; 3] = [
            Vertex2::<i32> {x: 0, y: 0},
            Vertex2::<i32> {x: 0, y: 0},
            Vertex2::<i32> {x: 0, y: 0},
        ];
        let mut world_coords: [Vertex3<f32>; 3] = [
            Vertex3::<f32> {x: 0f32, y: 0f32, z: 0f32},
            Vertex3::<f32> {x: 0f32, y: 0f32, z: 0f32},
            Vertex3::<f32> {x: 0f32, y: 0f32, z: 0f32},
        ];
        for i in 0..3 {
            let vertex_index = *face.get(i).unwrap() as usize;
            world_coords[i] = *model.verts.get(vertex_index).unwrap();
            screen_coords[i] = Vertex2::<i32> {
                x: ((world_coords[i].x + 1.0) * WIDTH as f32 / 2.0) as i32,
                y: ((world_coords[i].y + 1.0) * HEIGHT as f32 / 2.0) as i32
            } ;
        }
        let mut n = Vertex3::cross((world_coords[2] - world_coords[0]), (world_coords[1] - world_coords[0]));
        n = n.normalize();
        let intensity = n * light_dir;
        if intensity > 0.0 {
            let color_value = (255.0 * intensity) as u8;
            let pixel = image::Rgba([color_value, color_value, color_value, 255u8]);
            renderer::triangle(&screen_coords[0], &screen_coords[1], &screen_coords[2], &mut imgbuf, pixel);
        }
    }

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgba8(image::imageops::flip_vertical(&imgbuf)).save(fout, image::PNG);
}
