extern crate image;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use geometry::Vertex3;
use model::Model;
use renderer::GouradShader;
use std::f32;
use std::fs::File;
use std::path::Path;

pub mod geometry;
pub mod model;
pub mod renderer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const DEPTH: u32 = 255;
const ZBUFFER_SIZE: usize = ((WIDTH + 1) * (HEIGHT + 1)) as usize;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pocket-renderer", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // Draw a black canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Setup event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Load the model
    let model = Model::new("african_head");
    let light_dir = Vertex3::init(1.0, 1.0, 1.0).normalize();
    let up = Vertex3::init(0.0, 1.0, 0.0);
    let eye = Vertex3::init(1.0, 1.0, 3.0);
    let center = Vertex3::new();

    let projection = renderer::projection(eye, center);
    let viewport = renderer::viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4, DEPTH);
    let model_view = renderer::lookat(eye, center, up);
    let uniform_m = projection.clone() * model_view.clone();
    let uniform_mit = uniform_m.invert_transpose();

    let mut zbuffer: [f32; ZBUFFER_SIZE] = [f32::NEG_INFINITY; ZBUFFER_SIZE];

    // event loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    println!("Up arrow key pressed");
                }
                _ => {}
            }
        }

        // draw stuff
        for face in model.faces.clone() {
            let mut screen_coords: [Vertex3<f32>; 3] = [Vertex3::new(); 3];
            let mut world_coords: [Vertex3<f32>; 3] = [Vertex3::new(); 3];

            for i in 0..3 {
                let vertex_index = face.get_vertex(i) as usize;
                world_coords[i] = *model.verts.get(vertex_index).unwrap();
                screen_coords[i] = (viewport.clone()
                    * projection.clone()
                    * model_view.clone()
                    * world_coords[i].to_matrix())
                .to_vector();
            }
            let shader = GouradShader::new(&model, &face, light_dir);

            renderer::triangle(&screen_coords, shader, &mut zbuffer, &mut canvas)
        }

        canvas.present();
    }
}
