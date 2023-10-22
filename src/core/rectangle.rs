use crate::core::size::Size;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub fn from(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl<T: From<u16>> Rectangle<T> {
    pub fn zero() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
            width: 0.into(),
            height: 0.into(),
        }
    }
}

impl<T: From<u16>> Rectangle<T> {
    pub fn from_size(size: Size<T>) -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
            width: size.width,
            height: size.height,
        }
    }
}
