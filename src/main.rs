extern crate image;

use std::fs::File;
use std::path::Path;
use std::f32;
use image::ImageBuffer;
use model::Model;
use geometry::{Vertex2, Vertex3};

pub mod model;
pub mod geometry;
pub mod renderer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const ZBUFFER_SIZE: usize = ((WIDTH + 1) * (HEIGHT + 1)) as usize;

fn main() {
    // +1 hack to get over the out of bounds errors
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1);
    let model = Model::new("african_head");
    let light_dir = Vertex3::<f32> {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let mut zbuffer: [f32; ZBUFFER_SIZE] = [f32::NEG_INFINITY; ZBUFFER_SIZE];

    for face in model.faces {
        let mut screen_coords: [Vertex3<f32>; 3] = [Vertex3::<f32> {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 3];
        let mut world_coords: [Vertex3<f32>; 3] = [Vertex3::<f32> {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 3];
        let mut texture_coords: [Vertex2<f32>; 3] = [Vertex2::<f32> { x: 0.0, y: 0.0 }; 3];
        for i in 0..3 {
            let vertex_index = face.get_vertex(i) as usize;
            let texture_index = face.get_texture(i) as usize;
            world_coords[i] = *model.verts.get(vertex_index).unwrap();
            texture_coords[i] = *model.textures.get(texture_index).unwrap();
            screen_coords[i] = world_coords[i].to_screen(HEIGHT, WIDTH);
        }
        let mut n = Vertex3::cross((world_coords[2] - world_coords[0]),
                                   (world_coords[1] - world_coords[0]));
        n = n.normalize();
        let intensity = n * light_dir;
        if intensity > 0.0 {
            renderer::triangle(&screen_coords,
                               &texture_coords,
                               &model.texture_image,
                               &mut zbuffer,
                               intensity,
                               &mut imgbuf)
        }
    }

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgba8(image::imageops::flip_vertical(&imgbuf)).save(fout, image::PNG);
}
