use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;

use geometry::Vertex3;

pub struct Model {
    verts: Vec<Vertex3<u32>>
}

impl Model {

    pub fn new<P: AsRef<Path>>(path: P) -> Model {
        let verts: Vec<Vertex3<u32>> = Vec::new();;
        let file = File::open(path);
        let mut buf_reader = BufReader::new(file.unwrap());
        for line in buf_reader.lines() {
            let decoded_line = line.unwrap();
            let values: Vec<&str> = decoded_line.split(" ").collect();
            if values[0] == "v" {
                println!("x = {}, y = {}, z = {}", values[1], values[2], values[3]);
            }
        }
        Model { verts: verts }
    } 
}

