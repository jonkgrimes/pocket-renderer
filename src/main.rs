extern crate image;

use std::fs::File;
use std::path::Path;
use std::vec::Vec;
use image::{ImageBuffer, Pixel};

const WHITE: [u8; 3] = [255 as u8, 0 as u8, 0 as u8];

fn line<P: Pixel + 'static>(imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>) {
    imgbuf.put_pixel(45, 67, image::Rgb(WHITE));
}

fn main() {
    let mut imgbuf = ImageBuffer::new(100, 100);
    line(&mut imgbuf);
    let ref mut fout = File::create(&Path::new("rendered.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
