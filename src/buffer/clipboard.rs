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

    pub fn paste(&mut self) {
        // TODO: Improve performance (add all text instead iterate chars)
        if let Some(clipboard) = &self.clipboard {
            let mut new_line = false;
            for c in clipboard.text.clone().chars() {
                match c {
                    '\n' => {
                        self.split_line_after();
                        new_line = true;
                    }
                    ch => {
                        if new_line && self.cursor.x == 0 {
                            self.insert_char(ch);
                            self.move_left();
                            new_line = false;
                        } else {
                            self.insert_char_after(ch);
                        }
                    }
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
    fn paste_test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());
        buffer.lines.clear();
        buffer.lines = vec![String::from("12345"), String::from("67890")];
        buffer.cursor = Point::new(1, 2);
        buffer.clipboard = Some(Clipboard {
            is_line: false,
            text: String::from("123\n456\n789"),
        });
        buffer.paste();
        let expected = String::from("12345\n678123\n456\n78990");
        assert_eq!(expected, buffer.get_content());
    }
}
