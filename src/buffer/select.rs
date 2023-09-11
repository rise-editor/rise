use crate::buffer::{Buffer, BufferMode};
use crate::core::Point;

pub struct Select {
    pub start: Point<usize>,
}

impl Buffer {
    pub fn reverse_selection(&mut self) {
        if let BufferMode::Visual = self.mode {
            let x = self.cursor.x;
            let y = self.cursor.y;
            self.move_cursor(self.select.start.y, self.select.start.x);
            self.select.start.x = x;
            self.select.start.y = y;
        }
    }
}
