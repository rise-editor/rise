use std::cmp::min;

use crate::buffer::Buffer;

impl Buffer {
    pub fn move_cursor(&mut self, row: usize, column: usize) {
        self.cursor.y = min(row, self.get_row_count() - 1);
        self.cursor.x = min(column, self.get_line_max_cursor_x(self.cursor.y));

        if self.cursor.x < self.scroll.x {
            self.scroll.x = self.cursor.x
        } else if self.scroll.x + (self.visible_area.width as usize) <= self.cursor.x {
            self.scroll.x = self.cursor.x - (self.visible_area.width as usize) + 1;
        }

        if self.cursor.y < self.scroll.y {
            self.scroll.y = self.cursor.y;
        } else if self.scroll.y + (self.visible_area.height as usize) <= self.cursor.y {
            self.scroll.y = self.cursor.y - (self.visible_area.height as usize) + 1;
        }
    }


    pub fn move_left(&mut self) {
        if self.cursor.x > 0 {
            self.move_cursor(self.cursor.y, self.cursor.x - 1);
        }
    }

    pub fn move_right(&mut self) {
        self.move_cursor(self.cursor.y, self.cursor.x + 1);
    }

    pub fn move_up(&mut self) {
        if self.cursor.y > 0 {
            self.move_cursor(self.cursor.y - 1, self.cursor.x);
        }
    }

    pub fn move_down(&mut self) {
        self.move_cursor(self.cursor.y + 1, self.cursor.x);
    }

    pub fn move_first_row(&mut self) {
        self.move_cursor(0, self.cursor.x);
    }

    pub fn move_last_row(&mut self) {
        self.move_cursor(self.get_row_count() - 1, self.cursor.x);
    }

    pub fn move_first_column(&mut self) {
        self.move_cursor(self.cursor.y, 0);
    }

    pub fn move_last_column(&mut self) {
        self.move_cursor(self.cursor.y, self.get_line_max_cursor_x(self.cursor.y));
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        buffer::{mode::BufferMode, Buffer},
        core::{Point, Size},
    };

    fn create_buffer() -> Buffer {
        Buffer {
            mode: BufferMode::Normal,
            scroll: Point { x: 0, y: 0 },
            cursor: Point { x: 0, y: 0 },
            visible_area: Size {
                width: 5,
                height: 5,
            },
            lines: vec![],
        }
    }

    #[test]
    pub fn move_tests() {
        let mut buffer = create_buffer();
        buffer.lines.push(String::from("1234567890"));
        buffer.lines.push(String::from("abcde"));
        buffer.lines.push(String::new());

        buffer.move_up();
        buffer.move_up();
        buffer.move_up();
        assert_eq!(0, buffer.cursor.y);
        buffer.move_left();
        buffer.move_left();
        buffer.move_left();
        assert_eq!(0, buffer.cursor.x);
        buffer.move_down();
        buffer.move_down();
        buffer.move_down();
        assert_eq!(2, buffer.cursor.y);
        buffer.move_up();
        buffer.move_up();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        assert_eq!("45678", buffer.get_line_visible_text(0));
        assert_eq!("de", buffer.get_line_visible_text(1));
        assert_eq!("", buffer.get_line_visible_text(2));
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_right();
        buffer.move_down();
        assert_eq!(4, buffer.cursor.x);
        assert_eq!(1, buffer.cursor.y);
        buffer.move_first_column();
        assert_eq!("12345", buffer.get_line_visible_text(0));
        assert_eq!("abcde", buffer.get_line_visible_text(1));
        assert_eq!("", buffer.get_line_visible_text(2));
        buffer.move_up();
        buffer.move_up();
        buffer.move_last_column();
        assert_eq!("67890", buffer.get_line_visible_text(0));
        assert_eq!("", buffer.get_line_visible_text(1));
        assert_eq!("", buffer.get_line_visible_text(2));
    }

    #[test]
    pub fn move_last_column_test() {
        let mut buffer = create_buffer();
        buffer.lines.push(String::new());
        buffer.enter_insert_mode();
        buffer.insert_char('1');
        buffer.insert_char('2');
        buffer.insert_char('3');
        buffer.insert_char('4');
        buffer.insert_char('4');
        buffer.enter_normal_mode();
        buffer.move_last_column();
    }
}