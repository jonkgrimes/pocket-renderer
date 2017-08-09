extern crate image;

use geometry::{Vertex2, Vertex3, Matrix};
use model::{Model,Face};
use image::{DynamicImage, RgbImage};
use std::f32;

pub trait Shader {
    fn fragment(&self, bar: Vertex3<f32>, pixel: &mut image::Rgb<u8>) -> bool;
}

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
    fn fragment(&self, bar: Vertex3<f32>, pixel: &mut image::Rgb<u8>) -> bool {
        let intensity = self.varying_intensity * bar;
        let uv = (self.varying_uv[0] * bar.x) + (self.varying_uv[1] * bar.y) + (self.varying_uv[2] * bar.z);
        let texture_pixel = self.model.uv(uv);
        for i in 0..3 {
            pixel[i] = (texture_pixel[i] as f32 * intensity) as u8;
        }
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
    m.set(0,3, (x + w) as f32 / 2.0);
    m.set(1,3, (y + h) as f32 / 2.0);
    m.set(2,3, depth as f32 / 2.0);

    m.set(0,0, w as f32 / 2.0);
    m.set(1,1, h as f32 / 2.0);
    m.set(2,2, depth as f32 /2.0);
    m
}

pub fn triangle<S: Shader>(verts: &[Vertex3<f32>; 3],
                shader: S,
                texture_map: &DynamicImage,
                zbuffer: &mut [f32],
                imgbuf: &mut RgbImage) {
    let height = (imgbuf.height() - 1) as f32;
    let width = (imgbuf.width() - 1) as f32;
    let texture_buf = texture_map.as_rgb8().unwrap();
    let texture_buf_height = texture_buf.height();
    let texture_buf_width  = texture_buf.width();
    let mut bboxmin = Vertex2::<f32> {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };
    let mut bboxmax = Vertex2::<f32> {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
    };
    let clamp = Vertex2::<f32> {
        x: width,
        y: height,
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
            let zbuff_idx = (p.x + p.y * width) as usize;
            if zbuffer[zbuff_idx - 1] < p.z {
                let mut pixel = image::Rgb([255u8; 3]);
                shader.fragment(bc_screen, &mut pixel);
                zbuffer[zbuff_idx - 1] = p.z;
                imgbuf.put_pixel(p.x as u32, p.y as u32, pixel);
            }
        }
    }
}