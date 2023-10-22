use crate::{buffer::Buffer, core::point::Point};

impl Buffer {
    pub fn get_cursor_screen_pos(&self) -> Point<u16> {
        Point {
            x: self.text_area.x + (self.cursor.x - self.scroll.x) as u16,
            y: self.text_area.y + (self.cursor.y - self.scroll.y) as u16,
        }
    }
}
