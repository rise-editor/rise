use std::ops::{Add, Div, Mul, Sub};

use crate::core::size::Size;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T>
where
    T: Add + Sub + Mul + Div + Clone,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where
    T: Add + Sub + Mul + Div + Clone + Default,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: T::default(),
            height: T::default(),
        }
    }

    pub fn from_size(size: &Size<T>) -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: size.width.clone(),
            height: size.height.clone(),
        }
    }

    pub fn to_size(&self) -> Size<T> {
        Size {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
