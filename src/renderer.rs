use geometry::Vertex2;
use image::{ImageBuffer, Pixel};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn triangle<P: Pixel + 'static>(v0: &Vertex2<i32>, v1: &Vertex2<i32>, v2: &Vertex2<i32>, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    line(v0.x, v0.y, v1.x, v1.y, imgbuf, pixel);
    line(v1.x, v1.y, v2.x, v2.y, imgbuf, pixel);
    line(v0.x, v0.y, v2.x, v2.y, imgbuf, pixel);
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
        let temp_x = point0.x;
        let temp_y = point0.y;
        point0 = Point { x: point1.x, y: point1.y };
        point1 = Point { x: temp_x, y: temp_y };
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