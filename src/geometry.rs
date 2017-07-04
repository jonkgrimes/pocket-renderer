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
