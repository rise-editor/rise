use std::cmp::min;

pub struct EditableText {
    pub text: String,
    pub cursor_x: usize,
}

impl EditableText {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_x: 0,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.text.insert(self.cursor_x, ch);
        self.move_right();
    }

    pub fn delete_char_previous(&mut self) {
        if self.cursor_x > 0 {
            self.text.remove(self.cursor_x - 1);
            self.move_left();
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x < self.text.len() {
            self.text.remove(self.cursor_x);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x = self.cursor_x - 1;
        }
    }

    pub fn move_right(&mut self) {
        self.cursor_x = min(self.text.len(), self.cursor_x + 1);
    }

    pub fn reset(&mut self) {
        self.text = String::new();
        self.cursor_x = 0;
    }

    pub fn handle_key(&mut self, key: &str) {
        match key {
            "left" => self.move_left(),
            "right" => self.move_right(),
            "<c-h>" => self.delete_char_previous(),
            "backspace" => self.delete_char_previous(),
            "delete" => self.delete_char(),
            other => {
                if other.len() == 1 {
                    self.insert_char(other.chars().nth(0).unwrap());
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::core::editable_text::EditableText;

    #[test]
    fn test() {
        let mut editable = EditableText {
            text: String::new(),
            cursor_x: 0,
        };

        editable.reset();

        assert_eq!("", editable.text);
        assert_eq!(0, editable.cursor_x);

        editable.delete_char_previous();
        editable.move_left();
        editable.move_right();
        editable.insert_char('b');

        assert_eq!("b", editable.text);
        assert_eq!(1, editable.cursor_x);

        editable.move_left();

        assert_eq!(0, editable.cursor_x);

        editable.insert_char('a');

        assert_eq!("ab", editable.text);
        assert_eq!(1, editable.cursor_x);

        editable.move_right();
        editable.move_right();
        editable.insert_char('c');

        assert_eq!("abc", editable.text);
        assert_eq!(3, editable.cursor_x);
    }
}
