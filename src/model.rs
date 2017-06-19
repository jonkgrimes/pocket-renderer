use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;
use std::f32;

use geometry::Vertex3;

pub struct Model {
    verts: Vec<Vertex3<f32>>,
    faces: Vec<Vec<u32>>
}

impl Model {

    pub fn new<P: AsRef<Path>>(path: P) -> Model {
        let mut verts: Vec<Vertex3<f32>> = Vec::new();;
        let mut faces: Vec<Vec<u32>> = Vec::new();
        let file = File::open(path);
        let buf_reader = BufReader::new(file.unwrap());
        for line in buf_reader.lines() {
            let decoded_line = line.unwrap();
            let values: Vec<&str> = decoded_line.split(" ").collect();

            if values[0] == "v" {
                let x: f32 = values[1].parse().unwrap();
                let y: f32 = values[2].parse().unwrap();
                let z: f32 = values[3].parse().unwrap();
                verts.push(Vertex3 { x: x, y: y, z: z });
            }

            if values[0] == "f" {
                println!("{}", values[1]);
                let vert_index_list: Vec<u32> = values[1].split("/").map(|string| string.parse().unwrap());
                faces.push(vert_index_list);
            }
        }
        Model { verts: verts, faces: faces }
    } 

    pub fn verts_len(&self) -> usize {
        self.verts.len()
    }

    pub fn faces_len(&self) -> usize {
        self.faces.len()
    }
}

