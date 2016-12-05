extern crate num;

use std::ops::{Add, Mul};
use num::{Zero, One};

#[derive(Debug)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {

    pub fn new(x: T, y: T) -> Self {
        Vector2 { x: x, y: y }
    }

    pub fn origin() -> Self
        where T: Zero
    {
        Vector2 { x: T::zero(), y: T::zero() }
    }

    pub fn unit_x() -> Self
        where T: Zero + One
    {
        Vector2 { x: T::one(), y: T::zero() }
    }

    pub fn unit_y() -> Self
        where T: Zero + One
    {
        Vector2 { x: T::zero(), y: T::one() }
    }
}

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;
    fn add(self, other: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Mul + Clone> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;
    fn mul(self, other: T) -> Self::Output {
        Vector2 {
            x: self.x * other.clone(),
            y: self.y * other.clone(),
        }
    }
}

impl<T> PartialEq for Vector2<T>
    where T: PartialEq
{
    fn eq(&self, other: &Vector2<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

mod tests {

    use super::*;

    #[test]
    fn vector2_additiontest() {
        let a = Vector2::new(5, 7);
        let b = Vector2::new(6, 8);
        let c = Vector2::new(11, 15);
        assert_eq!((a + b), c);

    }

    #[test]
    fn vector2_multiplicationtest() {
        let e = Vector2::new(8, 6);
        let f = Vector2::new(4, 3);
        assert_eq!((f * 2), e);
    }

    #[test]
    fn origin_test() {
        let o = Vector2::new(0, 0);
        assert_eq!(o, Vector2::origin());
    }

    #[test]
    fn unitvec_test() {
        let x = Vector2::new(1, 0);
        let y = Vector2::new(0, 1);
        assert_eq!(x, Vector2::unit_x());
        assert_eq!(y, Vector2::unit_y());
    }
}
