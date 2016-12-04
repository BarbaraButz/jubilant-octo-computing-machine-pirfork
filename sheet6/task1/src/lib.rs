#[cfg(test)]
mod tests;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> Option<T> {
    if !(min <= max) {
        return None
    }

    if min <= val && val <= max {
        Some(val)
    } else {
        if val < min {
            Some(min)
        } else if val > max {
            Some(max)
        } else {
            None
        }
    }
}

use std::ops::{Add, Mul};
pub fn sum_and_product<T, U>(left: T, right: U) -> (<T as Add<U>>::Output, <T as Mul<U>>::Output)
    where T: Add<U> + Mul<U> + Clone, U: Clone
{
    (left.clone() + right.clone(), left * right)
}


trait IntoOptExt {
    fn into_option<T>(self, x: T) -> Option<T>;
}

impl IntoOptExt for bool {
    fn into_option<T>(self, x: T) -> Option<T> {
        if self { Some(x) } else { None }
    }
}
