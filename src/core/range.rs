use crate::core::point::Point;

pub struct Range<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T> Range<T> {
    pub fn new(start: Point<T>, end: Point<T>) -> Self {
        Self { start, end }
    }
}
