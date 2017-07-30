use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Vertex3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub m: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix { m: Vec::new() }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.m[x][y]
    }

    pub fn with_capacity(size: usize) -> Matrix {
        Matrix { m: vec![vec![0.0; size]; size] }
    }

    pub fn identity(size: usize) -> Matrix {
        let mut matrix = Matrix::with_capacity(size);
        for i in 0..size {
            for j in 0..size {
                match i == j {
                    true => matrix.m[i][j] = 1.0,
                    false => matrix.m[i][j] = 0.0,
                }
            }
        }
        matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let rows = self.m.len();
        let columns = self.m[0].len();
        for x in 0..rows {
            for y in 0..columns {
                match self.get(x,y) == other.get(x,y) {
                    true => return true,
                    false => return false
                }
            }
        }
        true
    }
}

pub struct Scalar {
    pub value: f32,
}

impl<T> Vertex2<T> {
    pub fn new(x: i32, y: i32) -> Vertex2<i32> {
        Vertex2 { x: x, y: y }
    }

    pub fn at(&self, i: i32) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            _ => None,
        }
    }
}

impl<T> Vertex3<T> {
    pub fn at(&self, i: i32) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }
}

impl Vertex3<f32> {
    pub fn to_screen(&self, height: u32, width: u32) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: ((self.x + 1.0) * width as f32 / 2.0) + 0.5,
            y: ((self.y + 1.0) * height as f32 / 2.0) + 0.5,
            z: self.z,
        }
    }
}

impl Add for Vertex2<i32> {
    type Output = Vertex2<i32>;

    fn add(self, other: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Vertex2<f32> {
    type Output = Vertex2<f32>;

    fn add(self, other: Vertex2<f32>) -> Vertex2<f32> {
        Vertex2::<f32> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vertex2<i32> {
    type Output = Vertex2<i32>;

    fn sub(self, other: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Vertex2<i32>> for Scalar {
    type Output = Vertex2<i32>;

    fn mul(self, rhs: Vertex2<i32>) -> Vertex2<i32> {
        Vertex2::<i32> {
            x: (self.value * rhs.x as f32) as i32,
            y: (self.value * rhs.y as f32) as i32,
        }
    }
}

impl Mul<Vertex3<f32>> for Scalar {
    type Output = Vertex3<f32>;

    fn mul(self, rhs: Vertex3<f32>) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: self.value * rhs.x,
            y: self.value * rhs.y,
            z: self.value * rhs.z,
        }
    }
}

impl Mul<Vertex3<f32>> for Vertex3<f32> {
    type Output = f32;

    fn mul(self, rhs: Vertex3<f32>) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Vertex3<f32> {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vertex3<f32> {
        Scalar { value: 1.0 / self.norm() } * *self
    }

    pub fn cross(u: Vertex3<f32>, v: Vertex3<f32>) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn barycentric(v0: Vertex3<f32>,
                       v1: Vertex3<f32>,
                       v2: Vertex3<f32>,
                       p: Vertex3<f32>)
                       -> Vertex3<f32> {
        let x: Vertex3<f32> = Vertex3::<f32> {
            x: v2.x - v0.x,
            y: v1.x - v0.x,
            z: v0.x - p.x,
        };
        let y: Vertex3<f32> = Vertex3::<f32> {
            x: v2.y - v0.y,
            y: v1.y - v0.y,
            z: v0.y - p.y,
        };
        let u = Vertex3::cross(x, y);
        if u.z.abs() < 1.0 {
            return Vertex3::<f32> {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            };
        }
        Vertex3::<f32> {
            x: 1.0 - (u.x + u.y) / u.z,
            y: u.y / u.z,
            z: u.x / u.z,
        }
    }
}

impl Vertex2<f32> {
    pub fn barycentric(v0: Vertex2<f32>,
                       v1: Vertex2<f32>,
                       v2: Vertex2<f32>,
                       p:  Vertex2<f32>) 
                       -> Vertex3<f32> {
        let a = Vertex3::<f32> { 
            x: v2.x - v0.x,
            y: v1.x - v0.x,
            z: v0.x - p.x
        };
        let b = Vertex3::<f32> {
            x: v2.y - v0.y,
            y: v1.y - v0.y,
            z: v0.y - p.y
        };
        let u = Vertex3::cross(a, b);
        if u.z.abs() < 1.0 {
            return Vertex3::<f32> {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            };
        }
        Vertex3::<f32> {
            x: 1.0 - (u.x + u.y) / u.z,
            y: u.y / u.z,
            z: u.x / u.z,
        }
    }
}

impl Sub for Vertex3<f32> {
    type Output = Vertex3<f32>;

    fn sub(self, rhs: Vertex3<f32>) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vertex2<f32>> for Vertex2<f32> {
    type Output = f32;

    fn mul(self, rhs: Vertex2<f32>) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f32> for Vertex2<f32> {
    type Output = Vertex2<f32>;

    fn mul(self, rhs: f32) -> Vertex2<f32> {
        Vertex2::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl PartialEq for Vertex2<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Vertex3<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_of_two_3d_vectors() {
        let a = Vertex3::<f32> {
            x: 3.0,
            y: 1.0,
            z: 0.0,
        };
        let b = Vertex3::<f32> {
            x: 1.0,
            y: 3.0,
            z: 1.0,
        };
        let actual = Vertex3::cross(a, b);
        let expected = Vertex3::<f32> {
            x: 1.0,
            y: -3.0,
            z: 8.0,
        };
        assert!(actual == expected);
    }

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

    #[test]
    fn matrix_equal() {
        let a = Matrix {m: vec!(vec!(1.0,0.0,0.0),vec!(0.0,1.0,0.0),vec!(0.0,0.0,1.0))};
        let b = Matrix {m: vec!(vec!(1.0,0.0,0.0),vec!(0.0,1.0,0.0),vec!(0.0,0.0,1.0))};
        assert!(a == b);
    }

    #[test]
    fn matrix_identity() {
        let actual = Matrix::identity(3);
        let expected = Matrix {m: vec!(vec!(1.0,0.0,0.0),vec!(0.0,1.0,0.0),vec!(0.0,0.0,1.0))};
        println!("{:?}", actual);
        println!("{:?}", expected);
        assert!(actual == expected);
    }
}
