use crate::buffer::Buffer;

impl Buffer {
    pub fn move_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.column < self.get_current_line_length() {
            self.cursor.column += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
        }

        if self.get_current_line_length() < self.cursor.column {
            self.move_line_end();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.row < self.get_lines_length() - 1 {
            self.cursor.row += 1;
        }

        if self.get_current_line_length() < self.cursor.column {
            self.move_line_end();
        }
    }

    pub fn move_line_start(&mut self) {
        self.cursor.column = 0;
    }

    pub fn move_line_end(&mut self) {
        self.cursor.column = self.get_current_line_length();
    }
}
