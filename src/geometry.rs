use std::fmt;
use std::f32;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

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
    rows: usize,
    columns: usize,
    pub m: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows: rows,
            columns: cols,
            m: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.m[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.m[x][y] = value
    }

    pub fn with_capacity(size: usize) -> Matrix {
        Matrix::new(size, size)
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

    pub fn invert_transpose(&self) -> Matrix {
        Matrix::identity(3)
    }

    pub fn to_vector(&self) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: self.get(0, 0) / self.get(3, 0),
            y: self.get(1, 0) / self.get(3, 0),
            z: self.get(2, 0) / self.get(3, 0),
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let rows = self.rows();
        let columns = self.columns();
        for x in 0..rows {
            for y in 0..columns {
                match self.get(x, y) == other.get(x, y) {
                    true => return true,
                    false => return false,
                }
            }
        }
        true
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        assert!(self.columns == rhs.rows);
        let mut result = Matrix::new(self.rows, rhs.columns);
        for i in 0..self.rows {
            for j in 0..rhs.columns {
                for k in 0..self.columns {
                    let value = result.get(i, j) + (self.get(i, k) * rhs.get(k, j));
                    result.set(i, j, value);
                }
            }
        }
        result
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
    pub fn new() -> Vertex3<f32> {
        Vertex3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn init(x: f32, y: f32, z: f32) -> Vertex3<f32> {
        Vertex3 { x: x, y: y, z: z }
    }

    pub fn to_screen(&self, height: u32, width: u32) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: ((self.x + 1.0) * width as f32 / 2.0) + 0.5,
            y: ((self.y + 1.0) * height as f32 / 2.0) + 0.5,
            z: self.z,
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        Matrix {
            rows: 4,
            columns: 1,
            m: vec![vec![self.x], vec![self.y], vec![self.z], vec![1.0]],
        }
    }
}

impl<T: Add<Output = T>> Add for Vertex2<T> {
    type Output = Vertex2<T>;

    fn add(self, other: Vertex2<T>) -> Vertex2<T> {
        Vertex2::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Vertex3<f32> {
    type Output = Vertex3<f32>;

    fn add(self, other: Vertex3<f32>) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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
                       p: Vertex2<f32>)
                       -> Vertex3<f32> {
        let a = Vertex3::<f32> {
            x: v2.x - v0.x,
            y: v1.x - v0.x,
            z: v0.x - p.x,
        };
        let b = Vertex3::<f32> {
            x: v2.y - v0.y,
            y: v1.y - v0.y,
            z: v0.y - p.y,
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

impl Mul<f32> for Vertex3<f32> {
    type Output = Vertex3<f32>;

    fn mul(self, rhs: f32) -> Vertex3<f32> {
        Vertex3::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl PartialEq for Vertex2<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Vertex3<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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
        let a = Matrix {
            rows: 3,
            columns: 3,
            m: vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
        };
        let b = Matrix {
            rows: 3,
            columns: 3,
            m: vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
        };
        assert!(a == b);
    }

    #[test]
    fn matrix_identity() {
        let actual = Matrix::identity(3);
        let expected = Matrix {
            rows: 3,
            columns: 3,
            m: vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
        };
        assert!(actual == expected);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Matrix {
            rows: 2,
            columns: 2,
            m: vec![vec![2.0, 1.0], vec![2.0, 2.0]],
        };
        let b = Matrix {
            rows: 2,
            columns: 1,
            m: vec![vec![2.0], vec![3.0]],
        };
        let actual = a * b;
        let expected = Matrix {
            rows: 2,
            columns: 1,
            m: vec![vec![7.0], vec![10.0]],
        };
        assert!(actual == expected);
    }

    #[test]
    fn matrix_invert_transpose() {
        let a = Matrix {
            rows: 3,
            columns: 3,
            m: vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
        };
        let actual = a.invert_transpose();
        let expected = Matrix {
            rows: 3,
            columns: 3,
            m: vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
        };
        assert!(actual == expected);
    }
}
