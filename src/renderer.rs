use image::{ImageBuffer, Pixel};

pub fn line<P: Pixel + 'static>(x0: u32, y0: u32, x1: u32, y1: u32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut t = 0.0; 
    while t < 1.0 {
        let x = x0 as f32 * (1.0 - t) + x1 as f32 * t;
        let y = y0 as f32 * (1.0 - t) + y1  as f32 * t;
        imgbuf.put_pixel(x as u32, y as u32, pixel);
        t += 0.01;
    }
}
