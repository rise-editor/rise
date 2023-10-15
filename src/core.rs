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
