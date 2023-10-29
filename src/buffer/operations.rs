use crate::buffer::Buffer;

impl Buffer {
    pub fn insert_char_to(&mut self, row: usize, column: usize, ch: char) -> Result<(), String> {
        let line = self.get_line_mut(row)?;

        if column > line.len() {
            return Err(format!(
                "Can not insert char at {} (line: {})",
                column,
                row + 1
            ));
        }

        line.insert(column, ch);

        Ok(())
    }

    pub fn insert_char_at_cursor(&mut self, ch: char) {
        let row = self.cursor.y;
        let column = self.cursor.x;
        self.insert_char_to(row, column, ch).unwrap();
        self.move_cursor(row, column + 1);
    }

    pub fn insert_char_at_after_cursor(&mut self, ch: char) {
        if self.get_current_line_text_length() == 0 {
            self.insert_char_at_cursor(ch);
        } else {
            let row = self.cursor.y;
            let column = self.cursor.x + 1;
            self.insert_char_to(row, column, ch).unwrap();
            self.move_cursor(row, column);
        }
    }

    pub fn insert_str_to(&mut self, row: usize, column: usize, text: &str) -> Result<(), String> {
        let line = self.get_line_mut(row)?;
        if column > line.len() {
            return Err(format!(
                "Can not insert str at {} (line: {})",
                column,
                row + 1
            ));
        }
        line.insert_str(column, text);
        Ok(())
    }

    pub fn insert_str_at_cursor(&mut self, text: &str) {
        let row = self.cursor.y;
        let column = self.cursor.x;
        self.insert_str_to(row, column, text).unwrap();
        self.move_cursor(row, column + text.len().checked_sub(1).unwrap_or(0));
    }

    pub fn insert_str_at_after_cursor(&mut self, text: &str) {
        if self.get_current_line_text_length() == 0 {
            self.insert_str_at_cursor(text);
        } else {
            let row = self.cursor.y;
            let column = self.cursor.x;
            self.insert_str_to(row, column + 1, text).unwrap();
            self.move_cursor(row, column + text.len());
        }
    }

    pub fn insert_whitespace_at_cursor(&mut self) {
        let row = self.cursor.y;
        let column = self.cursor.x;
        let text = self.options.get_whitespace_chars();
        self.insert_str_to(row, column, &text).unwrap();
        self.move_cursor(self.cursor.y, column + text.len());
    }

    pub fn delete_char_from(&mut self, row: usize, column: usize) -> Result<(), String> {
        let line = self.get_line_mut(row)?;
        if line.len() == column {
            if row + 1 < self.get_line_count() {
                self.join_lines(row, row + 1).unwrap();
            }
        } else if column > line.len() {
            return Err(format!("No column at {} (line: {})", column, row + 1));
        } else {
            line.remove(column);
            self.move_cursor(row, column);
            self.set_size(self.area.clone());
        }

        Ok(())
    }

    pub fn delete_char_from_cursor(&mut self) {
        self.delete_char_from(self.cursor.y, self.cursor.x).unwrap();
    }

    pub fn delete_previous_char_from(&mut self, row: usize, column: usize) -> Result<(), String> {
        if row == 0 && column == 0 {
            return Ok(());
        }

        if column == 0 {
            self.move_up();
            self.move_last_column();
            self.join_lines(row - 1, row)?;
        } else {
            self.move_left();
            self.delete_char_from_cursor();
        }

        Ok(())
    }

    pub fn delete_previous_char_from_cursor(&mut self) {
        self.delete_previous_char_from(self.cursor.y, self.cursor.x)
            .unwrap();
    }

    pub fn substitute_char_from_cursor(&mut self) {
        self.delete_char_from_cursor();
        self.enter_insert_mode();
    }

    fn insert_line_at(&mut self, row: usize) {
        self.lines.insert(row, String::new());
        self.set_size(self.area.clone());
    }

    pub fn open_new_line_previous(&mut self) {
        let row = self.cursor.y;
        self.insert_line_at(row);
        self.move_cursor(row, 0);
        self.enter_insert_mode();
    }

    pub fn open_new_line_next(&mut self) {
        let row = self.cursor.y + 1;
        self.insert_line_at(row);
        self.move_cursor(row, 0);
        self.enter_insert_mode();
    }

    pub fn split_line_at(&mut self, row: usize, column: usize) -> Result<(), String> {
        let line = self.get_line(row)?;
        if column > line.len() {
            return Err(format!("No column at {} (line: {})", column, row + 1));
        }
        let (left, right) = line.split_at(column);

        let left_string = left.to_string();
        let right_string = right.to_string();

        self.lines[row] = left_string;
        self.lines.insert(row + 1, right_string);

        self.move_cursor(row + 1, 0);
        self.set_size(self.area.clone());

        Ok(())
    }

    pub fn split_line_at_cursor(&mut self) {
        self.split_line_at(self.cursor.y, self.cursor.x).unwrap();
    }

    pub fn split_line_at_after_cursor(&mut self) {
        if self.get_current_line_text_length() == 0 {
            self.split_line_at_cursor();
        } else {
            self.split_line_at(self.cursor.y, self.cursor.x + 1)
                .unwrap();
        }
    }

    pub fn join_lines(&mut self, row1: usize, row2: usize) -> Result<(), String> {
        let line2 = self.get_line(row2)?.clone();
        let line1 = self.get_line_mut(row1)?;
        line1.push_str(&line2);
        self.lines.remove(row2);
        self.set_size(self.area.clone());

        Ok(())
    }

    pub fn join_lines_cursor(&mut self) {
        if self.cursor.y < self.get_line_count() - 1 {
            self.join_lines(self.cursor.y, self.cursor.y + 1).unwrap();
        }
    }
}
