use std::cmp::min;

use crate::buffer::{Buffer, BufferMode};

impl Buffer {
    pub fn get_line_visible_text(&self, row: usize) -> Option<String> {
        if self.get_line_count() <= row {
            return None;
        }

        let line = self.get_line(row);

        let start_index = self.scroll.x;
        if line.len() <= start_index {
            return Some(String::new());
        }
        let end_index = min(line.len(), self.scroll.x + (self.text_area.width as usize));

        Some(line[start_index..end_index].to_string())
    }

    pub fn get_line_max_cursor_x(&self, row: usize) -> usize {
        let line_length = self.get_line(row).len();

        if line_length == 0 {
            0
        } else if let BufferMode::Insert = self.mode {
            line_length
        } else {
            line_length - 1
        }
    }
}
