use image::{ImageBuffer, Pixel};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn line<P: Pixel + 'static>(x0: i32, y0: i32, x1: i32, y1: i32, imgbuf: &mut ImageBuffer<P, Vec<P::Subpixel>>, pixel: P) {
    let mut t = 0.0; 
    let mut steep = false;
    let mut point0 = Point { x: x0, y: y0 };
    let mut point1 = Point { x: x1, y: y1 };

    println!("Original values: {:?}, {:?}", point0, point1);

    if (x0-x1).abs() < (y0-y1).abs() { // if line is steep, transpose the image
        println!("Transposing image");
        point0 = Point { x: y0, y: x0  };
        point1 = Point { x: y1, y: x1  };
        println!("After transposition: {:?}, {:?}", point0, point1);
        steep = true;
    }

    if x0 > x1 { // make it left to right
        println!("Draw left to right");
        point0 = Point { x: x1, y: y1 };
        point1 = Point { x: x0, y: y0 };
        println!("After draw order: {:?}, {:?}", point0, point1);
    }

    println!("Before drawing: {:?}, {:?}", point0, point1);

    let mut x = 0;
    while x <= point1.x {
        let t = (x - point0.x) as f32/(point1.x - point0.x) as f32;
        let y = point0.y as f32 * (1.0 - t) + point1.y as f32 * t;
        println!("x: {}, y: {}", x, y);
        if steep {
            imgbuf.put_pixel(y as u32, x as u32, pixel);
        } else {
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
        x += 1;
    }
}
