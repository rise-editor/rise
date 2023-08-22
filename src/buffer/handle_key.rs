use crate::buffer::Buffer;

impl Buffer {
    pub fn insert_char(&mut self, ch: char) {
        let column = self.cursor.column as usize;

        let current_line = self.get_current_line_mut();
        current_line.insert(column, ch);

        self.move_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor.row == 0 && self.cursor.column == 0 {
            return;
        }

        if self.cursor.column == 0 {
            self.move_up();
            self.move_line_end();

            self.join_lines(self.cursor.row, self.cursor.row + 1);
        } else {
            let char_index = self.cursor.column - 1;
            let current_line = self.get_current_line_mut();
            current_line.remove(char_index as usize);

            self.move_left();
        }
    }

    pub fn insert_newline(&mut self) {
        let column = self.cursor.column as usize;
        let row = self.cursor.row as usize;

        let current_line = self.get_current_line();
        let (left, right) = current_line.split_at(column);

        let left_string = left.to_string();
        let right_string = right.to_string();

        self.lines[row] = left_string;
        self.lines.insert(row + 1, right_string);

        self.move_down();
        self.move_line_start();
    }

    pub fn join_lines(&mut self, row1: u32, row2: u32) {
        let line2 = self.get_line(row2).clone();
        let line1 = self.get_line_mut(row1);
        line1.push_str(&line2);
        self.lines.remove(row2 as usize);
    }
}
