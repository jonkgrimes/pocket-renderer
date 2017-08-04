extern crate image;

use std::fs::File;
use std::path::Path;
use std::f32;
use image::ImageBuffer;
use model::Model;
use geometry::{Vertex2, Vertex3, Matrix};

pub mod model;
pub mod geometry;
pub mod renderer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const DEPTH: u32 = 255;
const ZBUFFER_SIZE: usize = ((WIDTH + 1) * (HEIGHT + 1)) as usize;

fn main() {
    // +1 hack to get over the out of bounds errors
    let mut imgbuf = ImageBuffer::new(WIDTH + 1, HEIGHT + 1);
    let model = Model::new("african_head");
    let mut light_dir = Vertex3::<f32> { x: 1.0, y: -1.0, z: 1.0, };
    light_dir = light_dir.normalize();
    let eye = Vertex3::<f32> { x: 1.0, y: 1.0, z: 3.0, };
    let center = Vertex3::new();

    let mut projection = Matrix::identity(4);
    let viewport = renderer::viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3/4, HEIGHT * 3/4, DEPTH);
    let model_view = renderer::lookat(eye, center, Vertex3::<f32> { x: 0.0, y: 1.0, z: 0.0 });
    projection.set(3, 2, -1.0 / ((eye - center).norm()));

    let mut zbuffer: [f32; ZBUFFER_SIZE] = [f32::NEG_INFINITY; ZBUFFER_SIZE];

    for face in model.faces {
        let mut screen_coords: [Vertex3<f32>; 3] = [Vertex3::new(); 3];
        let mut world_coords: [Vertex3<f32>; 3] = [Vertex3::new(); 3];
        let mut normal_coords: [Vertex3<f32>; 3] = [Vertex3::new(); 3];
        let mut normals: [f32; 3] = [0.0; 3];
        let mut texture_coords: [Vertex2<f32>; 3] = [Vertex2::<f32> { x: 0.0, y: 0.0 }; 3];

        for i in 0..3 {
            let vertex_index = face.get_vertex(i) as usize;
            let texture_index = face.get_texture(i) as usize;
            let normal_index = face.get_normal(i) as usize;
            world_coords[i] = *model.verts.get(vertex_index).unwrap();
            texture_coords[i] = *model.textures.get(texture_index).unwrap();
            normal_coords[i] = *model.normals.get(normal_index).unwrap();
            normals[i] = normal_coords[i].normalize() * light_dir;
            screen_coords[i] = (viewport.clone() * projection.clone() * model_view.clone()  * world_coords[i].to_matrix()).to_vector();
        }

        renderer::triangle(&screen_coords,
                            &texture_coords,
                            &normals,
                            &model.texture_image,
                            &mut zbuffer,
                            &mut imgbuf)
    }

    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgba8(image::imageops::flip_vertical(&imgbuf)).save(fout, image::PNG);
}
