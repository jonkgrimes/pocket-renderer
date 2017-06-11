extern crate image;

use std::fs::File;
use std::path::Path;
use std::vec::Vec;
use image::{ImageBuffer, Pixel};

const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];

fn line<P: Pixel + 'static>(x0: u32, y0: u32, x1: u32, y1: u32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut t = 0.0; 
    while t < 1.0 {
        let x = x0 as f32 * (1.0 - t) + x1 as f32 * t;
        println!("{}", x);
        let y = y0 as f32 * (1.0 - t) + y1  as f32 * t;
        println!("{}", y);
        imgbuf.put_pixel(x as u32, y as u32, pixel);
        t += 0.01;
    }
}

fn main() {
    let mut imgbuf = ImageBuffer::new(100, 100);
    line(24, 56, 78, 45, &mut imgbuf, image::Rgb(WHITE));
    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
