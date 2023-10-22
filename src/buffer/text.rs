use crate::buffer::Buffer;

impl Buffer {
    pub fn get_line(&self, row: usize) -> &String {
        self.lines.get(row).unwrap()
    }

    pub fn get_line_mut(&mut self, row: usize) -> &mut String {
        self.lines.get_mut(row).unwrap()
    }

    pub fn get_current_line(&self) -> &String {
        self.get_line(self.cursor.y)
    }

    pub fn get_current_line_mut(&mut self) -> &mut String {
        self.get_line_mut(self.cursor.y)
    }

    pub fn get_line_text_length(&self, row: usize) -> usize {
        self.get_line(row).len()
    }

    pub fn get_current_line_text_length(&self) -> usize {
        self.get_current_line().len()
    }

    pub fn get_line_last_char_index(&self, row: usize) -> Option<usize> {
        match self.get_line_text_length(row) {
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
