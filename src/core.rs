pub mod editable;
pub mod key;

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Size<T> {
    pub width: T,
    pub height: T,
}
