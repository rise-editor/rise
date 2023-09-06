use std::cmp::{min, max};

pub struct CommandLine {
    pub text: String,
    pub cursor_x: usize,
}

impl CommandLine {
    pub fn insert_key(&mut self, ch: char) {
        self.text.insert(self.cursor_x, ch);
        self.move_right();
    }

    pub fn delete_key(&mut self) {
        if self.cursor_x > 0 {
            self.text.remove(self.cursor_x - 1);
            self.move_left();
        }
    }

    pub fn move_left(&mut self) {
        self.cursor_x = max(0, self.cursor_x - 1);
    }

    pub fn move_right(&mut self) {
        self.cursor_x = min(self.text.len(), self.cursor_x + 1);
    }

    pub fn reset(&mut self) {
        self.text = String::new();
        self.cursor_x = 0;
    }
}
