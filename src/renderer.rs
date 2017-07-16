extern crate image;

use geometry::Vertex2;
use geometry::Vertex3;
use image::{ImageBuffer, Pixel};
use std::mem;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn triangle<P: Pixel + 'static>(vert0: &Vertex2<i32>,
                                    vert1: &Vertex2<i32>,
                                    vert2: &Vertex2<i32>,
                                    zbuffer: &mut [f32],
                                    imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>,
                                    pixel: P) {
    let verts = [vert0, vert1, vert2];
    let mut bboxmin = Vertex2::<i32> {
        x: imgbuf.width() as i32 - 1,
        y: imgbuf.height() as i32 - 1,
    };
    let mut bboxmax = Vertex2::<i32> { x: 0, y: 0 };
    let clamp = Vertex2::<i32> {
        x: imgbuf.width() as i32 - 1,
        y: imgbuf.height() as i32 - 1,
    };
    for i in 0..3 {
        bboxmin.x = cmp::max(0, cmp::min(bboxmin.x, verts[i].x));
        bboxmax.x = cmp::min(clamp.x, cmp::max(bboxmax.x, verts[i].x));
        bboxmin.y = cmp::max(0, cmp::min(bboxmin.y, verts[i].y));
        bboxmax.y = cmp::min(clamp.y, cmp::max(bboxmax.y, verts[i].y));
    }

    let mut p = Vertex2::<i32> {
        x: bboxmin.x,
        y: bboxmin.y,
    };

    for x in bboxmin.x..(bboxmax.x + 1) {
        for y in bboxmin.y..(bboxmax.y + 1) {
            p.x = x;
            p.y = y;
            let bc_screen = Vertex3::barycentric(*vert0, *vert1, *vert2, p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            imgbuf.put_pixel(p.x as u32, p.y as u32, pixel);
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
