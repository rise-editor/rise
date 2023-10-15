use crate::buffer::Buffer;

impl Buffer {
    pub fn column_to_visible_x(&self, column: usize) -> u16 {
        self.text_area.width + (column - self.scroll.x) as u16
    }

    pub fn row_to_visible_y(&self, row: usize) -> u16 {
        self.text_area.height + (row - self.scroll.y) as u16
    }
}
