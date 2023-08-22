use crate::buffer::Buffer;

impl Buffer {
    pub fn get_line(&self, row: u32) -> &String {
        self.lines.get(row as usize).unwrap()
    }

    pub fn get_line_mut(&mut self, row: u32) -> &mut String {
        self.lines.get_mut(row as usize).unwrap()
    }

    pub fn get_current_line(&self) -> &String {
        self.get_line(self.cursor.row)
    }

    pub fn get_current_line_mut(&mut self) -> &mut String {
        self.get_line_mut(self.cursor.row)
    }

    pub fn get_lines_length(&self) -> u32 {
        self.lines.len() as u32
    }

    pub fn get_current_line_length(&self) -> u32 {
        self.get_line(self.cursor.row).len() as u32
    }

    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }
}
