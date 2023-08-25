pub mod handle_key;
pub mod mode;
pub mod move_cursor;
pub mod text;

use crate::core::{Point, Size};

use self::mode::BufferMode;

pub struct Buffer {
    pub mode: BufferMode,
    pub visible_area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn column_to_visible_x(&self, column: usize) -> u16 {
        (column - self.scroll.x) as u16
    }

    pub fn row_to_visible_y(&self, row: usize) -> u16 {
        (row - self.scroll.y) as u16
    }
}
