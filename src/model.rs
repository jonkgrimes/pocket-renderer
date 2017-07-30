extern crate image;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;
use std::f32;
use image::DynamicImage;

use geometry::Vertex2;
use geometry::Vertex3;

pub struct Model {
    pub verts: Vec<Vertex3<f32>>,
    pub textures: Vec<Vertex2<f32>>,
    pub faces: Vec<Face>,
    pub texture_image: DynamicImage,
}

#[derive(Debug)]
pub struct Face {
    pub vertexes: [u32; 3],
    pub textures: [u32; 3],
}

impl Face {
    pub fn get_vertex(&self, i: usize) -> u32 {
        self.vertexes[i]
    }

    pub fn get_texture(&self, i: usize) -> u32 {
        self.textures[i]
    }
}

impl Model {
    pub fn new(path: &str) -> Model {
        let mut verts: Vec<Vertex3<f32>> = Vec::new();
        let mut textures: Vec<Vertex2<f32>> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        let file = File::open(Path::new(&format!("models/{}.obj", path)));
        let buf_reader = BufReader::new(file.unwrap());
        let texture_image = image::open(Path::new(&format!("models/{}_diffuse.png", path)));
        let texture_image = match texture_image {
            Ok(file) => {
                println!("Loaded texture file...");
                file.flipv()
            },
            Err(error) => {
                panic!("There was a problems opening the texture file: {:?}", error)
            },
        };
        for line in buf_reader.lines() {
            let decoded_line = line.unwrap();
            let values: Vec<&str> = decoded_line.split(" ").collect();

            // parse out all the vertices
            if values[0] == "v" {
                let x: f32 = values[1].parse().unwrap();
                let y: f32 = values[2].parse().unwrap();
                let z: f32 = values[3].parse().unwrap();
                verts.push(Vertex3 { x: x, y: y, z: z });
            }

            // get the texture coordinates
            if values[0] == "vt" {
                let x: f32 = values[2].parse().unwrap();
                let y: f32 = values[3].parse().unwrap();
                textures.push(Vertex2 { x: x, y: y });
            }

            // parse out the faces which are of the following format
            // f vertex0_idx/texture_idx/normal_idx vertex1_idx/...
            if values[0] == "f" {
                let mut face = Face {
                    vertexes: [0; 3],
                    textures: [0; 3],
                };
                for i in 0..3 {
                    face.vertexes[i] = *parse_face_string(values[i + 1]).get(0).unwrap();
                    face.textures[i] = *parse_face_string(values[i + 1]).get(1).unwrap();
                }
                faces.push(face);
            }
        }
        Model {
            verts: verts,
            faces: faces,
            textures: textures,
            texture_image: texture_image,
        }
    }

    pub fn uv(&self, uv: Vertex2<f32>) -> &image::Rgba<u8> {
        let imgbuf = self.texture_image.as_rgba8().unwrap();
        let height = imgbuf.height();
        let width = imgbuf.width();
        imgbuf.get_pixel(uv.x as u32 * height, uv.y as u32 * width)
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
