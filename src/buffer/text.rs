use crate::buffer::Buffer;

impl Buffer {
    pub fn get_line(&self, row: usize) -> Result<&String, String> {
        match self.lines.get(row) {
            Some(line) => Ok(line),
            None => Err(format!("No line at {}", row)),
        }
    }

    pub fn get_line_mut(&mut self, row: usize) -> Result<&mut String, String> {
        match self.lines.get_mut(row) {
            Some(line) => Ok(line),
            None => Err(format!("No line at {}", row)),
        }
    }

    pub fn get_current_line(&self) -> &String {
        self.get_line(self.cursor.y).unwrap()
    }

    pub fn get_current_line_mut(&mut self) -> &mut String {
        self.get_line_mut(self.cursor.y).unwrap()
    }

    pub fn get_line_text_length(&self, row: usize) -> Result<usize, String> {
        Ok(self.get_line(row)?.len())
    }

    pub fn get_current_line_text_length(&self) -> usize {
        self.get_current_line().len()
    }

    pub fn get_line_last_char_index(&self, row: usize) -> Option<usize> {
        match self.get_line_text_length(row).unwrap() {
            0 => None,
            length => Some(length - 1),
        }
    }

    pub fn get_current_line_last_char_index(&self) -> Option<usize> {
        self.get_line_last_char_index(self.cursor.y)
    }

    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn set_content(&mut self, content: String) {
        self.lines = content.split('\n').map(|x| String::from(x)).collect();
        self.move_cursor(0, 0);
        self.set_size(self.area.clone());
    }
}
