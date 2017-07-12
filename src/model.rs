use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;
use std::f32;

use geometry::Vertex3;

pub struct Model {
    pub verts: Vec<Vertex3<f32>>,
    pub faces: Vec<Vec<u32>>,
}

impl Model {
    pub fn new<P: AsRef<Path>>(path: P) -> Model {
        let mut verts: Vec<Vertex3<f32>> = Vec::new();
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
                let mut vert_index_list: Vec<u32> = Vec::new();
                vert_index_list.push(*parse_face_string(values[1]).get(0).unwrap());
                vert_index_list.push(*parse_face_string(values[2]).get(0).unwrap());
                vert_index_list.push(*parse_face_string(values[3]).get(0).unwrap());
                faces.push(vert_index_list);
            }
        }
        Model {
            verts: verts,
            faces: faces,
        }
    }

    pub fn verts_len(&self) -> usize {
        self.verts.len()
    }

    pub fn faces_len(&self) -> usize {
        self.faces.len()
    }
}

fn parse_face_string(face_str: &str) -> Vec<u32> {
    let parse_index = |string: &str| {
        let value: u32 = string.parse().unwrap();
        value - 1 // waveform object files start index at 1 rather than 0
    };
    face_str.split("/").map(&parse_index).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_face_string_returns_correct_vector() {
        let str = "1/2/3";
        let expected = vec![0, 1, 2];
        let actual = parse_face_string(str);
        assert!(actual == expected);
    }
}
