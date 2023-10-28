use crate::core::point::Point;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextPosition {
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

impl TextPosition {
    pub fn get_start_point(&self) -> Point<usize> {
        Point::new(self.row, self.start)
    }

    pub fn get_end_point(&self) -> Point<usize> {
        Point::new(self.row, self.end)
    }
}
