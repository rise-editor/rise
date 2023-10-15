use crate::buffer::Buffer;

impl Buffer {
    pub fn insert_char(&mut self, ch: char) {
        self.insert_char_to(self.cursor.y, self.cursor.x, ch);
    }

    pub fn insert_char_to(&mut self, row: usize, column: usize, ch: char) {
        let line = self.get_line_mut(row);
        line.insert(column, ch);

        self.move_cursor(row, column + 1);
    }

    pub fn delete_char(&mut self) {
        self.delete_char_from(self.cursor.y, self.cursor.x);
    }

    pub fn delete_char_from(&mut self, row: usize, column: usize) {
        let line = self.get_line_mut(row);
        if line.len() == 0 {
            return;
        } else if line.len() == column {
            if row + 1 < self.get_row_count() {
                self.join_lines(row, row + 1);
            }
        } else {
            line.remove(column);
            self.move_cursor(row, column);
            self.set_size(self.area.clone());
        }
    }

    pub fn delete_char_before(&mut self, row: usize, column: usize) {
        if row == 0 && column == 0 {
            return;
        }

        if column == 0 {
            self.move_up();
            self.move_last_column();
            self.join_lines(row - 1, row);
        } else {
            self.move_left();
            self.delete_char();
        }
    }

    pub fn insert_newline(&mut self, row: usize) {
        self.lines.insert(row, String::new());
        self.move_cursor(row, 0);
        self.set_size(self.area.clone());
    }

    pub fn split_line(&mut self, row: usize, column: usize) {
        let line = self.get_line(row);
        let (left, right) = line.split_at(column);

        let left_string = left.to_string();
        let right_string = right.to_string();

        self.lines[row] = left_string;
        self.lines.insert(row + 1, right_string);

        self.move_cursor(row + 1, 0);
        self.set_size(self.area.clone());
    }

    pub fn join_lines(&mut self, row1: usize, row2: usize) {
        let line2 = self.get_line(row2).clone();
        let line1 = self.get_line_mut(row1);
        line1.push_str(&line2);
        self.lines.remove(row2);
        self.set_size(self.area.clone());
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn join_lines_test() {
    }
}
