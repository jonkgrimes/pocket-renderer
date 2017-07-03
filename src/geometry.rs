#[derive(Debug, Copy, Clone)]
pub struct Vertex3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vertex2<T> {
    pub fn new(x: i32, y: i32) -> Vertex2<i32> {
        Vertex2 { x: x, y: y }
    }
}