use std::cmp::min;

use crate::buffer::Buffer;

use super::mode::BufferMode;

impl Buffer {
    pub fn get_line(&self, row: usize) -> &String {
        self.lines.get(row).unwrap()
    }

    pub fn get_line_mut(&mut self, row: usize) -> &mut String {
        self.lines.get_mut(row).unwrap()
    }

    pub fn get_line_visible_text(&self, row: usize) -> String {
        let line = self.get_line(row);

        let start_index = self.scroll.x;
        if line.len() <= start_index {
            return String::new();
        }
        let end_index = min(
            line.len(),
            self.scroll.x + (self.visible_area.width as usize),
        );

        line[start_index..end_index].to_string()
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

    pub fn get_current_line(&self) -> &String {
        self.get_line(self.cursor.y)
    }

    pub fn get_current_line_mut(&mut self) -> &mut String {
        self.get_line_mut(self.cursor.y)
    }

    pub fn get_current_line_length(&self) -> usize {
        self.get_current_line().len()
    }

    pub fn get_row_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn set_content(&mut self, content: String) {
        self.lines = content.split('\n').map(|x| String::from(x)).collect();
        self.move_cursor(0, 0);
    }
}
