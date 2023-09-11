use crate::{buffer::Buffer, core::Point};

#[derive(PartialEq)]
enum CharType {
    WhiteSpace,
    AlphaNumeric,
    Special,
}

fn get_char_type(ch: char) -> CharType {
    if ch.is_whitespace() {
        CharType::WhiteSpace
    } else if ch.is_alphanumeric() {
        CharType::AlphaNumeric
    } else {
        CharType::Special
    }
}

impl Buffer {
    pub fn move_to_next_word_start(&mut self) {
        let cursor = self.find_next_word_start();
        self.move_cursor(cursor.y, cursor.x);
    }

    fn find_next_word_start(&self) -> Point<usize> {
        let current_char = self.get_line(self.cursor.y).chars().nth(self.cursor.x).unwrap();
        let mut current_char_type: CharType = get_char_type(current_char);
        let mut column_start_index = self.cursor.x + 1;

        for row in self.cursor.y..self.get_row_count() {
            let line = self.get_line(row);
            for column in column_start_index..line.len() {
                let char_type = get_char_type(line.chars().nth(column).unwrap());

                if char_type == CharType::WhiteSpace {
                    current_char_type = CharType::WhiteSpace;
                } else if current_char_type != char_type {
                    return Point { y: row, x: column };
                }
            }
            column_start_index = 0;
        }

        let last_line_index = self.get_row_count() - 1;
        let last_column_index = self.get_line_max_cursor_x(last_line_index);

        Point {
            y: last_line_index,
            x: last_column_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::{mode::BufferMode, Buffer, Select},
        command_line::CommandLine,
        core::{Point, Size},
    };

    fn create_buffer() -> Buffer {
        Buffer {
            file_name: None,
            mode: BufferMode::Normal,
            scroll: Point { x: 0, y: 0 },
            cursor: Point { x: 0, y: 0 },
            visible_area: Size {
                width: 5,
                height: 5,
            },
            lines: vec![],
            select: Select { start: Point { x: 0, y: 0 } },
            command_line: CommandLine {
                text: String::new(),
                cursor_x: 0,
            },
        }
    }

    #[test]
    fn find_next_word_start_test() {
        let mut buffer = create_buffer();
        buffer.lines.push(String::from("abc defg"));
        buffer.lines.push(String::from("123 123"));
        buffer.move_cursor(1, 2);

        let position = buffer.find_next_word_start();
        assert_eq!(1, position.y);
        assert_eq!(4, position.x);

        buffer.move_cursor(1, 4);
        buffer.lines.push(String::from(""));
        buffer.lines.push(String::from("     "));
        buffer.lines.push(String::from(""));
        buffer.lines.push(String::from("   123 123"));

        let position2 = buffer.find_next_word_start();
        assert_eq!(5, position2.y);
        assert_eq!(3, position2.x);
    }
}
