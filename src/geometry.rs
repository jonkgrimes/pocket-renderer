use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

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

pub struct Scalar {
    pub value: f32
}

impl<T> Vertex2<T> {
    pub fn new(x: i32, y: i32) -> Vertex2<i32> {
        Vertex2 { x: x, y: y }
    }

    pub fn at(&self, i: i32) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            _ => None
        }
    }
}

impl<T> Vertex3<T> {
    pub fn at(&self, i: i32) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None
        }
    }
}

impl Add for Vertex2<i32> {
    type Output = Vertex2<i32>;

    fn add(self, other: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vertex2<i32> {
    type Output = Vertex2<i32>;

    fn sub(self, other: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<Vertex2<i32>> for Scalar {
    type Output = Vertex2<i32>;

    fn mul(self, rhs: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: (self.value * rhs.x as f32) as i32,
            y: (self.value * rhs.y as f32) as i32
        }
    }
}

impl Vertex3<f32> {
    pub fn cross(u: Vertex3<f32>, v: Vertex3<f32>) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

    pub fn barycentric(v0: Vertex2<i32>, v1: Vertex2<i32>, v2: Vertex2<i32>, p: Vertex2<i32>) -> Vertex3<f32> {
        let x: Vertex3<f32> = Vertex3::<f32> { 
            x: v2.x as f32 - v0.x as f32,
            y: v1.x as f32 - v0.x as f32, 
            z: v0.x as f32 - p.x as f32
        };
        let y: Vertex3<f32> = Vertex3::<f32> { 
            x: v2.y as f32 - v0.y as f32,
            y: v1.y as f32 - v0.y as f32,
            z: v0.y as f32 - p.y as f32
        };
        let u = Vertex3::cross(x, y);
        if u.z.abs() < 1.0 {
            return Vertex3::<f32> { x: -1.0, y: 1.0, z: 1.0 };
        }
        Vertex3::<f32> {
            x: 1.0 - (u.x + u.y)/u.z,
            y: u.y/u.z,
            z: u.z/u.z
        }
    }
}

impl PartialEq for Vertex2<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_two_2d_vectors() {
        let a = Vertex2::<i32> { x: 1, y: 1 };
        let b = Vertex2::<i32> { x: 1, y: 1 };
        let actual = a + b;
        let expected = Vertex2::<i32> { x: 2, y: 2 };
        assert!(actual == expected);
    }

    #[test]
    fn multiplying_2d_vector_by_float() {
        let a = Vertex2::<i32> { x: 10, y: 10 };
        let b = Scalar { value: 0.5 };
        let actual = b * a;
        let expected = Vertex2::<i32> { x: 5, y: 5 };
        assert!(actual == expected);
    }


    #[test]
    fn subtracting_two_2d_vectors() {
        let a = Vertex2::<i32> { x: 3, y: 3 };
        let b = Vertex2::<i32> { x: 1, y: 1 };
        let actual = a - b;
        let expected = Vertex2::<i32> { x: 2, y: 2 };
        assert!(actual == expected);
    }
}
