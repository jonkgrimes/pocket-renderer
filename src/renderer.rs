extern crate image;

use geometry::{Matrix, Vertex2, Vertex3};
use image::RgbImage;
use model::{Face, Model};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::f32;

pub trait Shader {
    fn fragment(&self, bar: Vertex3<f32>, pixel: &mut Color) -> bool;
}

pub struct Scene {}

pub struct GouradShader<'a> {
    varying_intensity: Vertex3<f32>,
    varying_uv: [Vertex3<f32>; 3],
    model: &'a Model,
}

impl<'a> GouradShader<'a> {
    pub fn new(model: &'a Model, face: &Face, light_dir: Vertex3<f32>) -> GouradShader<'a> {
        let mut intensity: [f32; 3] = [1.0; 3];
        let mut textures = [Vertex3::new(); 3];
        for i in 0..3 {
            let normal_idx = face.get_normal(i) as usize;
            let texture_idx = face.get_texture(i) as usize;
            let normal = *model.normals.get(normal_idx).unwrap();
            textures[i] = *model.textures.get(texture_idx).unwrap();
            intensity[i] = 0f32.max(normal * light_dir);
        }
        GouradShader {
            varying_intensity: Vertex3::init(intensity[0], intensity[1], intensity[2]),
            varying_uv: textures,
            model: model,
        }
    }
}

impl<'a> Shader for GouradShader<'a> {
    fn fragment(&self, bar: Vertex3<f32>, pixel: &mut Color) -> bool {
        let intensity = self.varying_intensity * bar;
        let uv = (self.varying_uv[0] * bar.x)
            + (self.varying_uv[1] * bar.y)
            + (self.varying_uv[2] * bar.z);
        let texture_pixel = self.model.uv(uv);
        pixel.r = (texture_pixel[0] as f32 * intensity) as u8;
        pixel.g = (texture_pixel[1] as f32 * intensity) as u8;
        pixel.b = (texture_pixel[2] as f32 * intensity) as u8;
        true
    }
}

pub fn lookat(eye: Vertex3<f32>, center: Vertex3<f32>, up: Vertex3<f32>) -> Matrix {
    let z = (eye - center).normalize();
    let x = Vertex3::cross(up, z).normalize();
    let y = Vertex3::cross(z, x).normalize();
    let mut result = Matrix::identity(4);
    for i in 0..3 {
        result.set(0, i, *x.at(i as i32).unwrap());
        result.set(1, i, *y.at(i as i32).unwrap());
        result.set(2, i, *z.at(i as i32).unwrap());
        result.set(i, 3, -center.at(i as i32).unwrap());
    }
    result
}

pub fn projection(eye: Vertex3<f32>, center: Vertex3<f32>) -> Matrix {
    let mut result = Matrix::identity(4);
    result.set(3, 2, -1.0 / ((eye - center).norm()));
    result
}

pub fn viewport(x: u32, y: u32, h: u32, w: u32, depth: u32) -> Matrix {
    let mut m = Matrix::identity(4);
    m.set(0, 3, (x + w) as f32 / 2.0);
    m.set(1, 3, (y + h) as f32 / 2.0);
    m.set(2, 3, depth as f32 / 2.0);

    m.set(0, 0, w as f32 / 2.0);
    m.set(1, 1, h as f32 / 2.0);
    m.set(2, 2, depth as f32 / 2.0);
    m
}

pub fn triangle<S: Shader>(
    verts: &[Vertex3<f32>; 3],
    shader: S,
    zbuffer: &mut [f32],
    canvas: &mut WindowCanvas,
) {
    let (height, width) = canvas.output_size().unwrap();
    let mut bboxmin = Vertex2::<f32> {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };
    let mut bboxmax = Vertex2::<f32> {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
    };
    let clamp = Vertex2::<f32> {
        x: width as f32,
        y: height as f32,
    };

    for i in 0..3 {
        bboxmin.x = 0f32.max(bboxmin.x.min(verts[i].x));
        bboxmax.x = clamp.x.min(bboxmax.x.max(verts[i].x));
        bboxmin.y = 0f32.max(bboxmin.y.min(verts[i].y));
        bboxmax.y = clamp.y.min(bboxmax.y.max(verts[i].y));
    }

    let mut p = Vertex3::<f32> {
        x: bboxmin.x as f32,
        y: bboxmin.y as f32,
        z: 0.0,
    };

    for x in (bboxmin.x as u32)..(bboxmax.x as u32 + 1) {
        for y in (bboxmin.y as u32)..(bboxmax.y as u32 + 1) {
            p.x = x as f32;
            p.y = y as f32;
            let bc_screen = Vertex3::barycentric(verts[0], verts[1], verts[2], p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            p.z = 0.0;
            p.z += verts[0].z * bc_screen.x;
            p.z += verts[1].z * bc_screen.y;
            p.z += verts[2].z * bc_screen.z;
            let zbuff_idx = (p.x + p.y * (width as f32)) as usize;
            if zbuffer[zbuff_idx - 1] < p.z {
                let mut pixel = Color::RGB(255u8, 255u8, 255u8);
                shader.fragment(bc_screen, &mut pixel);
                zbuffer[zbuff_idx - 1] = p.z;
                canvas.set_draw_color(pixel);
                canvas.draw_point(Point::new(p.x as i32, p.y as i32)).ok();
                canvas.present();
            }
        }
    }
}
