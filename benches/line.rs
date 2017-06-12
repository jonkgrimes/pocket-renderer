#[macro_use]
extern crate bencher;
extern crate image;
extern crate renderer;

use std::fs::File;
use std::path::Path;
use image::{ImageBuffer};
use bencher::Bencher;

const RED: [u8; 3] = [255 as u8, 0 as u8, 0 as u8];
const WHITE: [u8; 3] = [255 as u8, 255 as u8, 255 as u8];
const BLUE: [u8; 3] = [0 as u8, 0 as u8, 255 as u8];

fn line_benchmark(bench: &mut Bencher) {
    bench.iter(|| {
        let mut imgbuf = ImageBuffer::new(100, 100);
        for i in (1..1000000) {
            renderer::line(13, 20, 80, 40, &mut imgbuf, image::Rgb(RED));
            renderer::line(20, 13, 40, 80, &mut imgbuf, image::Rgb(BLUE));
            renderer::line(1, 13, 42, 73, &mut imgbuf, image::Rgb(WHITE));
        }
        let ref mut fout = File::create(&Path::new("benchmark.png")).unwrap();
        let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
    })
}

benchmark_group!(benches, line_benchmark);
benchmark_main!(benches);