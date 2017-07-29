extern crate image;

use geometry::Vertex2;
use geometry::Vertex3;
use image::{DynamicImage, ImageBuffer, RgbaImage, Pixel};
use std::mem;
use std::f32;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn triangle(verts: &[Vertex3<f32>; 3],
                textures: &[Vertex2<f32>; 3],
                texture_map: &DynamicImage,
                zbuffer: &mut [f32],
                intensity: f32,
                imgbuf: &mut RgbaImage) {
    let height = (imgbuf.height() - 1) as f32;
    let width = (imgbuf.width() - 1) as f32;
    let texture_buf = texture_map.as_rgba8().unwrap();
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
    let mut p_uv = Vertex2::<f32> {
        x: bboxmin.x as f32,
        y: bboxmin.y as f32
    };

    for x in (bboxmin.x as u32)..(bboxmax.x as u32 + 1) {
        for y in (bboxmin.y as u32)..(bboxmax.y as u32 + 1) {
            p.x = x as f32;
            p.y = y as f32;
            let bc_screen = Vertex3::barycentric(verts[0], verts[1], verts[2], p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            p_uv = textures[0] * bc_screen.x + textures[1] * bc_screen.y + textures[2] * bc_screen.z;

            p.z = 0.0;
            p.z += verts[0].z * bc_screen.x;
            p.z += verts[1].z * bc_screen.y;
            p.z += verts[2].z * bc_screen.z;
            let zbuff_idx = (p.x + p.y * width) as usize;
            if zbuffer[zbuff_idx - 1] < p.z {
                zbuffer[zbuff_idx - 1] = p.z;
                let texture_x = p_uv.x as u32 * texture_buf_height;
                let texture_y = p_uv.y as u32 * texture_buf_width;
                let texture_pixel = texture_buf.get_pixel(texture_x, texture_y);
                let pixel = image::Rgba([texture_pixel[0] * intensity as u8, texture_pixel[1] * intensity as u8, texture_pixel[2] * intensity as u8, 255u8]);
                imgbuf.put_pixel(p.x as u32, p.y as u32, pixel);
            }
        }
    }
}

pub fn line<P: Pixel + 'static>(x0: i32,
                                y0: i32,
                                x1: i32,
                                y1: i32,
                                imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>,
                                pixel: P) {
    let mut steep = false;
    let mut point0 = Point { x: x0, y: y0 };
    let mut point1 = Point { x: x1, y: y1 };

    if (x0 - x1).abs() < (y0 - y1).abs() {
        // if line is steep, transpose the image
        point0 = Point { x: y0, y: x0 };
        point1 = Point { x: y1, y: x1 };
        steep = true;
    }

    if point0.x > point1.x {
        // make it left to right
        mem::swap(&mut point0, &mut point1);
    }

    let dx = point1.x - point0.x;
    let dy = point1.y - point0.y;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;

    let mut x = point0.x;
    let mut y = point0.y;

    while x <= point1.x {
        if steep {
            imgbuf.put_pixel(y as u32, x as u32, pixel);
        } else {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
        error2 += derror2;
        if error2 > dx {
            if point1.y > point0.y {
                y += 1;
            } else {
                y += -1;
            }
            error2 -= dx * 2;
        }
        x += 1;
    }
}
