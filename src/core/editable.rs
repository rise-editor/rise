use std::cmp::min;

pub struct Editable {
    pub text: String,
    pub cursor_x: usize,
}

impl Editable {
    pub fn insert_key(&mut self, ch: char) {
        self.text.insert(self.cursor_x, ch);
        self.move_right();
    }

    pub fn delete_key(&mut self) {
        if self.cursor_x > 0 {
            self.text.remove(self.cursor_x - 1);
            self.move_left();
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
}

#[cfg(test)]
mod tests {
    use crate::core::editable::Editable;

    #[test]
    fn test() {
        let mut editable = Editable {
            text: String::new(),
            cursor_x: 0,
        };

        editable.reset();

        assert_eq!("", editable.text);
        assert_eq!(0, editable.cursor_x);

        editable.delete_key();
        editable.move_left();
        editable.move_right();
        editable.insert_key('b');

        assert_eq!("b", editable.text);
        assert_eq!(1, editable.cursor_x);

        editable.move_left();

        assert_eq!(0, editable.cursor_x);

        editable.insert_key('a');

        assert_eq!("ab", editable.text);
        assert_eq!(1, editable.cursor_x);

        editable.move_right();
        editable.move_right();
        editable.insert_key('c');

        assert_eq!("abc", editable.text);
        assert_eq!(3, editable.cursor_x);
    }
}
