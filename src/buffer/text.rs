use crate::{
    buffer::Buffer,
    core::{point::Point, text_reader::TextReader},
};

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

    pub fn get_text(&self, location1: Point<usize>, location2: Point<usize>) -> String {
        let (from, to) = Point::order(location1, location2);

        let mut result: Vec<String> = Vec::new();

        let mut reader = TextReader::new(&self.lines);
        let _ = reader.set_cursor(from);

        while reader.get_cursor() <= to {
            let mut line = String::new();
            if let Some(ch) = reader.get_char() {
                line.push(ch);
            }
            while !reader.is_line_last_x() && reader.get_cursor() < to {
                if let Some(ch) = reader.next() {
                    line.push(ch);
                }
            }
            result.push(line);
            reader.next();
        }

        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::Buffer,
        core::{point::Point, size::Size},
    };

    #[test]
    fn get_text_test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());
        buffer.lines.clear();
        buffer.lines.push(String::from("123456"));
        buffer.lines.push(String::from("qwerty"));
        buffer.lines.push(String::from(""));
        buffer.lines.push(String::from("qwe"));

        let lines = buffer.get_text(Point::new(0, 3), Point::new(3, 1));
        let lines2 = buffer.get_text(Point::new(3, 1), Point::new(0, 3));

        assert_eq!("456\nqwerty\n\nqw", lines);
        assert_eq!("456\nqwerty\n\nqw", lines2);
    }
}
