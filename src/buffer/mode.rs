use super::Buffer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BufferMode {
    Normal,
    Insert,
    Visual,
    Command,
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

    pub fn enter_visual_mode(&mut self) {
        self.mode = BufferMode::Visual;
        self.selection.start.x = self.cursor.x;
        self.selection.start.y = self.cursor.y;
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = BufferMode::Command;
    }
}
