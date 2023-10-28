use crate::buffer::{mode::BufferMode, Buffer};

pub struct Clipboard {
    pub text: String,
    pub is_line: bool,
}

impl Buffer {
    pub fn copy_selection(&mut self) {
        if let BufferMode::Visual = self.mode {
            self.clipboard = Some(Clipboard {
                is_line: false,
                text: self.get_text(self.selection.start.clone(), self.cursor.clone()),
            });
            self.enter_normal_mode();
        }
    }

    pub fn paste_at_cursor(&mut self) {
        if let Some(clipboard) = &self.clipboard {
            let clipboard_text = clipboard.text.clone();
            let lines = clipboard_text.split('\n');
            let mut is_first = true;

            for line in lines {
                if is_first {
                    is_first = false;
                } else {
                    self.split_line_at_after_cursor();
                }
                self.insert_str_at_cursor(line);
            }
        }
    }

    pub fn paste_at_after_cursor(&mut self) {
        if let Some(clipboard) = &self.clipboard {
            let clipboard_text = clipboard.text.clone();
            let lines = clipboard_text.split('\n');
            let mut is_first = true;

            for line in lines {
                if is_first {
                    self.insert_str_at_after_cursor(line);
                    is_first = false;
                } else {
                    self.split_line_at_after_cursor();
                    let _ = self.insert_str_to(self.cursor.y, 0, line);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::{clipboard::Clipboard, Buffer},
        core::{point::Point, size::Size},
    };

    #[test]
    fn paste_cursor_test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());
        buffer.lines.clear();
        buffer.lines = vec![String::from("12345"), String::from("67890")];
        buffer.cursor = Point::new(1, 2);
        buffer.clipboard = Some(Clipboard {
            is_line: false,
            text: String::from("123\n456\n789"),
        });
        buffer.paste_at_cursor();
        let expected = String::from("12345\n67123\n456\n789890");
        assert_eq!(expected, buffer.get_content());
    }

    #[test]
    fn paste_after_test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());
        buffer.lines.clear();
        buffer.lines = vec![String::from("12345"), String::from("67890")];
        buffer.cursor = Point::new(1, 2);
        buffer.clipboard = Some(Clipboard {
            is_line: false,
            text: String::from("123\n456\n789"),
        });
        buffer.paste_at_after_cursor();
        let expected = String::from("12345\n678123\n456\n78990");
        assert_eq!(expected, buffer.get_content());
    }
}
