extern crate image;

use geometry::{Vertex2, Vertex3, Matrix};
use image::{DynamicImage, RgbImage};
use std::f32;

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

pub fn triangle(verts: &[Vertex3<f32>; 3],
                textures: &[Vertex2<f32>; 3],
                intensity: &[f32; 3],
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
            let p_uv = textures[0] * bc_screen.x + textures[1] * bc_screen.y + textures[2] * bc_screen.z;
            let mut p_intensity = intensity[0] * bc_screen.x + intensity[1] * bc_screen.y + intensity[2] * bc_screen.z;
            p_intensity = 0f32.max(p_intensity.min(1.0));
            p.z = 0.0;
            p.z += verts[0].z * bc_screen.x;
            p.z += verts[1].z * bc_screen.y;
            p.z += verts[2].z * bc_screen.z;
            let zbuff_idx = (p.x + p.y * width) as usize;
            if zbuffer[zbuff_idx - 1] < p.z {
                let texture_x = p_uv.x * texture_buf_height as f32;
                let texture_y = p_uv.y * texture_buf_width as f32;
                let texture_pixel = texture_buf.get_pixel(texture_x as u32, texture_y as u32);
                let pixel = image::Rgb([(texture_pixel[0] as f32 * p_intensity) as u8,
                                        (texture_pixel[1] as f32 * p_intensity) as u8, 
                                        (texture_pixel[2] as f32 * p_intensity) as u8]);
                zbuffer[zbuff_idx - 1] = p.z;
                imgbuf.put_pixel(p.x as u32, p.y as u32, pixel);
            }
        }
    }
}