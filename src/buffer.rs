pub mod handle_key;
pub mod move_cursor;
pub mod text;

use std::fmt::Display;

use crate::cursor::Cursor;

#[derive(Debug)]
pub enum BufferMode {
    Normal,
    Insert,
    Visual,
}

impl Display for BufferMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Buffer {
    pub mode: BufferMode,
    pub cursor: Cursor,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn enter_normal_mode(&mut self) {
        self.mode = BufferMode::Normal;
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = BufferMode::Insert;
    }
}
