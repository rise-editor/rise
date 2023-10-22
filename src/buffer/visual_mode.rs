use crate::buffer::{Buffer, BufferMode};
use crate::core::Point;

pub struct Selection {
    pub start: Point<usize>,
}

impl Buffer {
    pub fn reverse_selection(&mut self) {
        if let BufferMode::Visual = self.mode {
            let x = self.cursor.x;
            let y = self.cursor.y;
            self.move_cursor(self.selection.start.y, self.selection.start.x);
            self.selection.start.x = x;
            self.selection.start.y = y;
        }
    }
}
