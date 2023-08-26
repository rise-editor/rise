use super::Buffer;

impl Buffer {
    pub fn column_to_visible_x(&self, column: usize) -> u16 {
        (column - self.scroll.x) as u16
    }

    pub fn row_to_visible_y(&self, row: usize) -> u16 {
        (row - self.scroll.y) as u16
    }
}
