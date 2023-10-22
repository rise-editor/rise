pub mod editable_text;
pub mod key;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

pub type Color = (u8, u8, u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

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
