extern crate image;

use geometry::Vertex2;
use geometry::Scalar;
use image::{ImageBuffer, Pixel};
use std::mem;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn triangle<P: Pixel + 'static>(vert0: &Vertex2<i32>, vert1: &Vertex2<i32>, vert2: &Vertex2<i32>, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut v0 = vert0;
    let mut v1 = vert1;
    let mut v2 = vert2;

    if v0.y > v1.y {
        mem::swap(&mut v0, &mut v1);
    }
    if v0.y > v2.y {
        mem::swap(&mut v0, &mut v2);
    }
    if v1.y > v2.y {
        mem::swap(&mut v1, &mut v2);
    }

    let total_height = v2.y - v0.y;
    // bottom half of the triangle
    for y in v0.y..(v1.y + 1) {
        let segment_height = v1.y - v0.y + 1;
        let alpha = Scalar { value: (y - v0.y) as f32 / total_height as f32 };
        let beta  = Scalar { value: (y - v0.y) as f32 / segment_height as f32 };
        let mut a: Vertex2<i32> = *v0 + (alpha * (*v2 - *v0));
        let mut b: Vertex2<i32> = *v0 + (beta * (*v1 - *v0));
        if a.x > b.x {
            mem::swap(&mut a, &mut b);
        }
        for x in a.x..(b.x + 1) {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // top half
    for y in v1.y..(v2.y + 1) {
        let segment_height = v2.y - v1.y + 1;
        let alpha = Scalar { value: (y - v0.y) as f32 / total_height as f32 };
        let beta  = Scalar { value: (y - v1.y) as f32 / segment_height as f32 };
        let mut a: Vertex2<i32> = *v0 + (alpha * (*v2 - *v0));
        let mut b: Vertex2<i32> = *v1 + (beta * (*v2 - *v1));
        if a.x > b.x {
            mem::swap(&mut a, &mut b);
        }
        for x in a.x..(b.x + 1) {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
    }
}

pub fn line<P: Pixel + 'static>(x0: i32, y0: i32, x1: i32, y1: i32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut steep = false;
    let mut point0 = Point { x: x0, y: y0 };
    let mut point1 = Point { x: x1, y: y1 };

    if (x0-x1).abs() < (y0-y1).abs() { // if line is steep, transpose the image
        point0 = Point { x: y0, y: x0  };
        point1 = Point { x: y1, y: x1  };
        steep = true;
    }

    if point0.x > point1.x { // make it left to right
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