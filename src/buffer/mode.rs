use std::fmt::{Display, Formatter, Result};

use super::Buffer;

#[derive(Debug)]
pub enum BufferMode {
    Normal,
    Insert,
    Visual,
}

impl Display for BufferMode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Buffer {
    pub fn enter_normal_mode(&mut self) {
        self.mode = BufferMode::Normal;
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = BufferMode::Insert;
    }
}
