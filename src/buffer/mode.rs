use std::fmt::{Display, Formatter, Result};

use super::Buffer;

#[derive(Debug, PartialEq)]
pub enum BufferMode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Display for BufferMode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Buffer {
    pub fn enter_normal_mode(&mut self) {
        if let BufferMode::Insert = self.mode {
            if self.cursor.x > 0 {
                self.move_cursor(self.cursor.y, self.cursor.x - 1);
            }
        }

        self.mode = BufferMode::Normal;
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = BufferMode::Insert;
    }

    pub fn enter_insert_mode_after(&mut self) {
        self.mode = BufferMode::Insert;
        self.move_right();
    }

    pub fn enter_visual_mode(&mut self) {
        self.mode = BufferMode::Visual;
        self.select.start.x = self.cursor.x;
        self.select.start.y = self.cursor.y;
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = BufferMode::Command;
        self.command_line.reset();
    }
}
