use image::{ImageBuffer, Pixel};
use std::mem;

pub fn line<P: Pixel + 'static>(x0: u32, y0: u32, x1: u32, y1: u32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut t = 0.0; 
    let mut steep = false;

    if ((x0-x1).abs() < (y0-y1).abs()) { // if line is steep, transpose the image
        mem::swap(x0, y0);
        mem::swap(x1, y1);
        steep = true;
    }

    if (x0 > x1) {
        mem::swap(x0, x1);
        mem::swap(y0, y1);
    }

    for x in (x0..x1) {
        let t = (x - x0) as f32/(x1 - x0) as f32;
        let y = y0 * (1.0 - t) + y1 as f32 * t;
        if (steep) {
            imgbuf.put_pixel(y as u32, x as u32, pixel);
        } else {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
    }
}
