use image::{ImageBuffer, Pixel};

pub fn line<P: Pixel + 'static>(x0: i32, y0: i32, x1: i32, y1: i32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut t = 0.0; 
    let mut steep = false;
    let (temp_x0, temp_y0) = (x0, y0);
    let (temp_x1, temp_y1) = (x1, y1);

    if (x0-x1).abs() < (y0-y1).abs() { // if line is steep, transpose the image
        let (x0, x1) = (y0, y1);
        let (y0, y1) = (temp_x0, temp_x1);
        println!("x0 = {} y0 = {}, x1 = {} y1 = {}", x0, y0, x1, y1);
        steep = true;
    }

    if x0 > x1 { // make it left to right
        let temp_x0 = x0;
        let temp_y0 = y0;
        let x0 = x1; let y0 = y1;
        let y1 = temp_y0; let x1 = temp_x0;
        println!("x0 = {} y0 = {}, x1 = {} y1 = {}", x0, y0, x1, y1);
    }

    println!("x0 = {} y0 = {}, x1 = {} y1 = {}", x0, y0, x1, y1);

    let mut x = 0;
    while x <= x1 {
        let t = (x - x0) as f32/(x1 - x0) as f32;
        let y = y0 as f32 * (1.0 - t) + y1 as f32 * t;
        println!("x = {}, y = {}",x,y);
        if steep {
            imgbuf.put_pixel(y as u32, x as u32, pixel);
        } else {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
        x += 1;
    }
}
