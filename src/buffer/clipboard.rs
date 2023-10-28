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
        if let Some(clipboard) = &self.clipboard {
            for c in clipboard.text.clone().chars() {
                match c {
                    '\n' => self.split_line_cursor(),
                    ch => self.insert_char_after(ch),
                }
            }
        }
    }
}
