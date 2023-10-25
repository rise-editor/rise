use std::ops::{Add, Div, Mul, Sub};

use crate::core::rectangle::Rectangle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Size<T>
where
    T: Add + Sub + Mul + Div + Clone,
{
    pub width: T,
    pub height: T,
}

impl<T> Size<T>
where
    T: Add + Sub + Mul + Div + Clone + Default,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: T::default(),
            height: T::default(),
        }
    }

    pub fn from_rectangle(rectangle: &Rectangle<T>) -> Self {
        Self {
            width: rectangle.width.clone(),
            height: rectangle.height.clone(),
        }
    }

    pub fn to_rectangle(&self) -> Rectangle<T> {
        Rectangle::from_size(self)
    }
}
